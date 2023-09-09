use crossbeam::channel::{unbounded, Receiver, Sender};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{str, thread, time};

use std::net::TcpStream;
use std::time::Duration;

use crate::sway_ipc_connection::Sway;
use crate::whitelist::get_white_list;

mod app_init;
mod overlay;
mod sway_ipc_connection;
mod kanata_io;

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

            let sx1 = sender.clone();
            let sx2 = sender.clone();

            thread::spawn(move || kanata_io::read_from_kanata(reader_stream, sx1));

            match sway_conn {
                Ok(sway) => {
                    #[cfg(feature = "overlay")]
                    let rx1 = receiver.clone();
                    #[cfg(feature = "overlay")]
                    thread::spawn(|| overlay::overlay::render_ovrelay(rx1));

                    let rx2 = receiver.clone();
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

fn main_loop(mut s: TcpStream, mut sway: Sway, receiver: Arc<Mutex<Receiver<String>>>, sender: Arc<Mutex<Sender<String>>>) {
    let mut cur_layer = String::from("main");

    let mut wait: bool = false;
    loop {
        log::warn!("Current time: {:?}", time::Instant::now());
                std::thread::sleep(Duration::from_millis(101));
        if wait {
            // Sleep for 1.5 seconds to prevent overheat
            std::thread::sleep(Duration::from_millis(101));
        }

        let layer_changed = receiver.lock().unwrap().try_recv();
        log::warn!("rsnd");

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
        if cur_win_name.is_none() {
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
            kanata_io::write_to_kanata(cur_win_name.unwrap(), &mut s, sender.clone());
        } else {
            log::trace!("should not change the layer");
            // TODO: Extract to configuration
            let default_layer = String::from("main");
            kanata_io::write_to_kanata(default_layer, &mut s, sender.clone());
        }
    }
}

fn should_change_layer(cur_win_name: String) -> bool {
    // PERF: Early exit, when found or cache on startup, which creates reload problems
    // TODO: Make the files path configurable
    let file_names: Vec<String> = glob("/home/iz/.config/keyboard/apps/*")
        .expect("Failed to read glob pattern")
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

    file_names.contains(&cur_win_name)
}
