use crossbeam::channel::{unbounded, Receiver, Sender};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{str, thread};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;

use crate::sway_ipc_connection::Sway;
use crate::whitelist::get_white_list;

mod app_init;
mod overlay;
mod sway_ipc_connection;
pub mod whitelist;

fn main() {
    let (kanata_conn, sway_conn) = app_init::init();

    match kanata_conn {
        Ok(kanata) => {
            let writer_stream = kanata.try_clone().expect("clone writer");
            let reader_stream = kanata;

            // Async cross-channel cammunication
            // Used to send the current layout from `kanata_reader` to `kanata_writer`
            // When telling kanata to change layout there are some checks for the current layout (unstable)
            let (sender, receiver) = unbounded::<String>();

            let receiver = Arc::new(Mutex::new(receiver));
            let sender = Arc::new(Mutex::new(sender));

            let rx1 = receiver.clone();
            let rx2 = receiver.clone();

            let sx1 = sender.clone();
            let sx2 = sender.clone();

            thread::spawn(move || read_from_kanata(reader_stream, sx1));

            match sway_conn {
                Ok(sway) => {
                    // TODO: Opt-out with config
                    thread::spawn(|| overlay::overlay::render_ovrelay(rx1));
                    main_loop(writer_stream, sway, rx2, sx2);
                }
                Err(e) => {
                    log::error!("Cannot connect to sway: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Cannot connect to kanata: {}", e);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    LayerChange { new: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    ChangeLayer { new: String },
}

impl FromStr for ServerMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

fn main_loop(mut s: TcpStream, mut sway: Sway, receiver: Arc<Mutex<Receiver<String>>>, sender: Arc<Mutex<Sender<String>>>) {
    let mut cur_layer = String::from("main");

    let mut wait: bool = false;
    loop {
        if wait {
            // Sleep for 1.5 seconds to prevent overheat
            std::thread::sleep(Duration::from_millis(1500));
        }
        let layer_changed = receiver.lock().unwrap().recv();

        // Returns ok when there is a layer change
        // Error if nothing changed
        if let Ok(new_layer) = layer_changed {
            log::warn!(
                "Received layer change SIGNAL: {} -> {}",
                cur_layer,
                new_layer
            );
            cur_layer = new_layer;

            wait = false;
        }

        let cur_win_name = sway.current_application();
        log::info!("Current app: {:?}", cur_win_name);
        if let None = cur_win_name {
            log::debug!("No app focused!");

            // Don't run the loop forever when no app is focused, fixes the overheat problem
            wait = true;
            continue;
        }

        log::trace!("Current layer: {}", cur_layer);

        // Don't change layer if not in a whitelisted file
        if let Some(whitelist) = get_white_list() {
            if !whitelist.contains(&cur_layer) {
                log::info!("Skipping {} because not in whitelist", &cur_layer);
                wait = true;
                continue;
            }
        }

        let should_change = should_change_layer(cur_win_name.clone().unwrap());
        if should_change {
            log::trace!("should change the layer");
            write_to_kanata(cur_win_name.unwrap(), &mut s, sender.clone());
        } else {
            log::trace!("should not change the layer");
            // TODO: Extract to configuration
            let default_layer = String::from("main");
            write_to_kanata(default_layer, &mut s, sender.clone());
        }

        std::thread::sleep(Duration::from_millis(1500));
    }
}

fn should_change_layer(cur_win_name: String) -> bool {
    // PERF: Early exit, when found or cache on startup, which creates reload problems
    // TODO: Make the files path configurable
    let file_names: Vec<String> = glob("/home/iz/.config/keyboard/apps/*")
        .expect("Failed to read glob pattern")
        .into_iter()
        .map(|e| {
            let val = e
                .expect("glob element")
                .file_name()
                .expect("file name")
                .to_str()
                .unwrap()
                .to_owned();

            log::debug!("File found: {}", val);
            val
        })
        .collect();

    return file_names.contains(&cur_win_name);
}

fn write_to_kanata(new: String, s: &mut TcpStream, sender: Arc<Mutex<Sender<String>>>) {
    log::info!("writer: telling kanata to change layer to \"{new}\"");
    sender.lock().unwrap().send(new.clone()).expect("send to reader");

    let msg = serde_json::to_string(&ClientMessage::ChangeLayer { new }).expect("deserializable");
    let expected_wsz = msg.len();

    let wsz = s.write(msg.as_bytes()).expect("stream writable");
    if wsz != expected_wsz {
        panic!("failed to write entire message {wsz} {expected_wsz}");
    }
}

fn read_from_kanata(mut s: TcpStream, sender: Arc<Mutex<Sender<String>>>) {
    log::info!("reader starting");
    let mut buf = vec![0; 256];
    loop {
        let sz = s.read(&mut buf).expect("stream readable");
        let msg = String::from_utf8_lossy(&buf[..sz]);
        let parsed_msg = ServerMessage::from_str(&msg).expect("kanata sends valid message");
        match parsed_msg {
            ServerMessage::LayerChange { new } => {
                log::info!("reader: KANATA CHANGED layers to \"{}\"", new);
                sender
                    .lock().expect("lock sender")
                    .send(new.clone())
                    .expect("send layer change to other proccess");
            }
        }
    }
}
