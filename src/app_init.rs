use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

use crate::sway_ipc_connection::Sway;

use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

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

fn connect_to_sway() -> Sway {
    let mut sway = Sway::new();
    sway.connect();
    sway
}

fn connect_to_kanata(args: Args) -> TcpStream {
    log::info!("attempting to CONNECT to kanata");
    let kanata_conn = TcpStream::connect_timeout(
        &SocketAddr::from(([127, 0, 0, 1], args.port)),
        Duration::from_secs(5),
    )
    .expect("connect to kanata");
    log::info!("successfully CONNECTED");
    kanata_conn
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
    log::info!("kanata_connection v{} starting", env!("CARGO_PKG_VERSION"));
}

fn configure_logger() -> Args {
    let args = Args::parse();
    init_logger(&args);
    args
}

pub(crate) fn init() -> (TcpStream, Sway) {
    let args = configure_logger();
    let kanata_conn = connect_to_kanata(args);
    let sway_connection = connect_to_sway();
    (kanata_conn, sway_connection)
}

