use clap::Parser;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::fs::read_dir;
use std::os::unix::prelude::OsStrExt;
use std::{env, str};

use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::Duration;

use swayipc::Connection;

struct Sway {
    connection: Option<Connection>,
}

impl Sway {
    pub fn new() -> Sway {
        Sway { connection: None }
    }

    fn connect(&mut self) {
        if let None = self.connection {
            if let Err(env::VarError::NotPresent) = env::var("SWAYSOCK") {
                let path = match find_socket() {
                    Some(path) => path,
                    None => {
                        println!("Failed to locate a SWAYSOCK from /run/user/1000/sway-ipc.*");
                        return;
                    }
                };
                println!("$SWAYSOCK is not set. Defaulting to \"{}\"", path);
                env::set_var("SWAYSOCK", path);
            }

            match Connection::new() {
                Ok(connection) => self.connection = Some(connection),
                Err(e) => println!("SwayClient#connect() failed: {}", e),
            }
        }
    }

    fn current_application(&mut self) -> Option<String> {
        self.connect();
        let connection = match &mut self.connection {
            Some(connection) => connection,
            None => return None,
        };

        if let Ok(node) = connection.get_tree() {
            if let Some(node) = node.find_focused(|n| n.focused) {
                if node.app_id.is_some() {
                    return node.app_id;
                } else if let Some(wp) = node.window_properties {
                    return wp.class;
                }
            }
        }
        None
    }
}

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

    let sway = Sway::new();
    read_from_kanata(writer_stream);
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

fn read_from_kanata(mut s: TcpStream) {
    loop {
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn _read_from_kanata_old(mut s: TcpStream) {
    loop {
        let result = get_sway_wininfo();
        let deserialized = serde_json::from_str::<Vec<WinInfo>>(&result).unwrap();

        // TODO: Only get focused window to remove the for loop
        for win in deserialized.iter() {
            if win.is_focused {
                // PERF: Optimize
                let layer = win.name.clone().trim().to_lowercase().to_owned();

                write_to_kanata(layer, &mut s);
            }
        }

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

fn get_sway_wininfo() -> String {
    let output = Command::new("swaymsg")
        .arg("--raw")
        .arg("-t")
        .arg("get_tree")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let jq_command = "[.nodes[].nodes[].nodes[] | {name: .name, is_focused: .focused}]";

    let jq_output = Command::new("jq")
        .arg(jq_command)
        .arg("--raw-output")
        .stdin(Stdio::from(output.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute jq command");

    let output = jq_output.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    result.to_owned()
}
