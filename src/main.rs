use crossbeam::channel::{unbounded, Receiver, Sender};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{str, thread};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;

use crate::sway_ipc_connection::Sway;
use crate::whitelist::get_white_list;

mod app_init;
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
            thread::spawn(move || read_from_kanata(reader_stream, sender));

            match sway_conn {
                Ok(sway) => {
                    main_loop(writer_stream, sway, receiver);
                },
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

fn main_loop(mut s: TcpStream, mut sway: Sway, receiver: Receiver<String>) {
    let mut cur_layer = String::from("main");

    let mut wait: bool = false;
    loop {
        if wait {
            // Sleep for 1.5 seconds to prevent overheat
            std::thread::sleep(Duration::from_millis(1500));
        }
        let layer_changed = receiver.recv();

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
        if let None = cur_win_name {
            log::warn!("No app focused!");

            // Don't run the loop forever when no app is focused, fixes the overheat problem
            wait = true;
            continue;
        }

        log::info!("CURRENT layer: {}", cur_layer);

        // Don't change layer if in a whitelisted file
        if let Some(whitelist) = get_white_list() {
            if whitelist.contains(&cur_layer) {
                log::warn!("---SKIPPING {}---", &cur_layer);
                wait = true;
                continue;
            }
        }

        let should_change = should_change_layer(cur_win_name.clone().unwrap());
        if should_change {
            log::warn!("SHOULD CHANGE");
            write_to_kanata(cur_win_name.unwrap(), &mut s);
        } else {
            log::warn!("FALLBACK");
            // TODO: Extract to configuration
            let default_layer = String::from("main");
            write_to_kanata(default_layer, &mut s);
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

fn write_to_kanata(new: String, s: &mut TcpStream) {
    log::info!("writer: telling kanata to change layer to \"{new}\"");
    let msg = serde_json::to_string(&ClientMessage::ChangeLayer { new }).expect("deserializable");
    let expected_wsz = msg.len();
    let wsz = s.write(msg.as_bytes()).expect("stream writable");
    if wsz != expected_wsz {
        panic!("failed to write entire message {wsz} {expected_wsz}");
    }
}

fn read_from_kanata(mut s: TcpStream, sender: Sender<String>) {
    log::info!("reader starting");
    let mut buf = vec![0; 256];
    loop {
        log::info!("reader: waiting for message from kanata");

        let sz = s.read(&mut buf).expect("stream readable");
        let msg = String::from_utf8_lossy(&buf[..sz]);
        let parsed_msg = ServerMessage::from_str(&msg).expect("kanata sends valid message");
        match parsed_msg {
            ServerMessage::LayerChange { new } => {
                log::info!("reader: KANATA CHANGED layers to \"{}\"", new);
                sender
                    .send(new.clone())
                    .expect("send layer change to other proccess");
            }
        }
    }
}
