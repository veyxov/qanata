use super::ServerMessage;

use super::ClientMessage;

use crossbeam::channel::Sender;

use std::io::Read;
use std::io::Write;
use std::str::FromStr;
use std::sync::Mutex;

use std::sync::Arc;

use std::net::TcpStream;

impl FromStr for ServerMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

pub fn write_to_kanata(new: String, s: &mut TcpStream, sender: Arc<Mutex<Sender<String>>>) {
    log::info!("writer: telling kanata to change layer to \"{new}\"");
    sender.lock().unwrap().send(new.clone()).expect("send to reader");

    let msg = serde_json::to_string(&ClientMessage::ChangeLayer { new }).expect("deserializable");
    let expected_wsz = msg.len();

    let wsz = s.write(msg.as_bytes()).expect("stream writable");
    if wsz != expected_wsz {
        panic!("failed to write entire message {wsz} {expected_wsz}");
    }
}

pub fn read_from_kanata(mut s: TcpStream, sender: Arc<Mutex<Sender<String>>>) {
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

