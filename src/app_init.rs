use anyhow::bail;
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
pub struct Args {
    /// Port that kanata's TCP server is listening on
    #[clap(short, long, default_value_t=7070)]
    pub port: u16,

    /// Enable debug logging
    #[clap(short, long)]
    pub debug: bool,

    /// Enable trace logging (implies --debug as well)
    #[clap(short, long)]
    pub trace: bool,

    /// Applications that should be ignored (don't change layer based on app)
    #[clap(short, long)]
    pub white_list_file: Option<String>,
}

fn connect_to_sway() -> anyhow::Result<Sway> {
    let mut sway = Sway::new();
    sway.connect();

    if sway.connection.is_none() {
        bail!("Could not connect to sway");
    }

    Ok(sway)
}

fn connect_to_kanata(args: Args) -> anyhow::Result<TcpStream> {
    log::info!("Connecting to kanata...");
    let kanata_conn = TcpStream::connect_timeout(
        &SocketAddr::from(([127, 0, 0, 1], args.port)),
        Duration::from_secs(5),
    )?;

    log::info!("...connected");

    Ok(kanata_conn)
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
}

fn configure_logger() -> Args {
    let args = Args::parse();
    init_logger(&args);
    args
}

pub(crate) fn init() -> (anyhow::Result<TcpStream>, anyhow::Result<Sway>) {
    let args = configure_logger();
    let kanata_conn = connect_to_kanata(args);
    let sway_connection = connect_to_sway();
    (kanata_conn, sway_connection)
}
