use clap::Parser;
use crossbeam::channel::{unbounded, Receiver, Sender};
use glob::glob;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::fs::read_dir;
use std::os::unix::prelude::OsStrExt;
use std::{str, thread};

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

use crate::sway_ipc_connection::Sway;

mod sway_ipc_connection;

fn find_socket() -> Option<String> {
    let uid = 1000;
    if let Some(run_user) = read_dir(format!("/run/user/{}", uid)).as_mut().ok() {
        while let Some(entry) = run_user.next() {
            let path = entry.ok()?.path();
            if let Some(fname) = path.file_name() {
                if fname.as_bytes().starts_with(b"sway-ipc.") {
                    if let Ok(path) = path.into_os_string().into_string() {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

#[derive(Debug, Serialize, Deserialize)]
struct WinInfo {
    name: String,
    is_focused: bool,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Port that kanata's TCP server is listening on
    #[clap(short, long)]
    port: u16,

    /// Enable debug logging
    #[clap(short, long)]
    debug: bool,

    /// Enable trace logging (implies --debug as well)
    #[clap(short, long)]
    trace: bool,
}

fn main() {
    let args = Args::parse();
    init_logger(&args);
    log::info!("attempting to connect to kanata");
    let kanata_conn = TcpStream::connect_timeout(
        &SocketAddr::from(([127, 0, 0, 1], args.port)),
        Duration::from_secs(5),
    )
    .expect("connect to kanata");
    log::info!("successfully connected");
    let writer_stream = kanata_conn.try_clone().expect("clone writer");
    let reader_stream = kanata_conn;

    let mut sway = Sway::new();
    sway.connect();

    // Async channell cammunication
    let (sender, receiver) = unbounded::<String>();
    thread::spawn(move || read_from_kanata(reader_stream, sender));

    main_loop(writer_stream, sway, receiver);
}

fn init_logger(args: &Args) {
    let log_lvl = match (args.debug, args.trace) {
        (_, true) => LevelFilter::Trace,
        (true, false) => LevelFilter::Debug,
        (false, false) => LevelFilter::Info,
    };
    let mut log_cfg = ConfigBuilder::new();
    if let Err(e) = log_cfg.set_time_offset_to_local() {
        eprintln!("WARNING: could not set log TZ to local: {:?}", e);
    };
    CombinedLogger::init(vec![TermLogger::new(
        log_lvl,
        log_cfg.build(),
        TerminalMode::Mixed,
        ColorChoice::AlwaysAnsi,
    )])
    .expect("init logger");
    log::info!(
        "kanata_example_tcp_client v{} starting",
        env!("CARGO_PKG_VERSION")
    );
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

    loop {
        let layer_changed = receiver.recv();

        // Returns ok when there is a layer change
        // Error if nothing changed
        if let Ok(new_layer) = layer_changed {
            log::warn!("Layer change: {}", new_layer);
            cur_layer = new_layer;
        }

        // If not in the main layer, don't change
        if cur_layer != "main" {
            log::warn!("Not in main layer, skipping");
            continue;
        }

        let cur_win_name = sway.current_application().unwrap();

        let should_change = should_change_layer(cur_win_name.clone(), &receiver);
        if should_change {
            log::warn!("can change layer to {}", cur_win_name);
            write_to_kanata(cur_win_name, &mut s);
        } else {
            log::warn!(
                "app specific layer for {} not found, fallback to default",
                cur_win_name
            );

            // TODO: Extract to configuration
            let default_layer = String::from("main");
            write_to_kanata(default_layer, &mut s);
        }

        std::thread::sleep(Duration::from_millis(1500));
    }
}

fn should_change_layer(cur_win_name: String, receiver: &Receiver<String>) -> bool {
    // PERF: Early exit, when found or cache on startup, which creates reload problems
    let file_names: Vec<String> = glob("/home/iz/.config/keyboard/apps/*")
        .expect("Failed to read glob pattern")
        .into_iter()
        .map(|e| {
            //files.push(e.unwrap().display());
            let val = e.unwrap().file_name().unwrap().to_str().unwrap().to_owned();

            log::warn!("File found: {}", val);
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
                log::info!("reader: kanata changed layers to \"{}\"", new);
                sender.send(new.clone()).unwrap();
            }
        }
    }
}
