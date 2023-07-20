use super::find_socket;
use swayipc::Connection;


use std::env;

pub struct Sway {
    pub(crate) connection: Option<Connection>,
}

impl Sway {
    pub fn new() -> Sway {
        Sway { connection: None }
    }

    pub(crate) fn connect(&mut self) {
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

    pub(crate) fn current_application(&mut self) -> Option<String> {
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
