use clap::Parser;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::fs::read_dir;
use std::os::unix::prelude::OsStrExt;
use std::{str};

use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

mod SwayIpcConnection;

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

    let mut sway = SwayIpcConnection::Sway::new();
    sway.connect();
    main_loop(writer_stream, sway);
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

fn main_loop(mut s: TcpStream, mut sway: SwayIpcConnection::Sway) {
    loop {
        let cur_win_name = sway.current_application().unwrap();

        write_to_kanata(cur_win_name, &mut s);

        std::thread::sleep(Duration::from_millis(500));
    }
}


fn write_to_kanata(new: String, s: &mut TcpStream) {
    //log::error!("focused window: {}", win.name);

    log::info!("writer: telling kanata to change layer to \"{new}\"");
    let msg = serde_json::to_string(&ClientMessage::ChangeLayer { new }).expect("deserializable");
    let expected_wsz = msg.len();
    let wsz = s.write(msg.as_bytes()).expect("stream writable");
    if wsz != expected_wsz {
        panic!("failed to write entire message {wsz} {expected_wsz}");
    }
}
