#![allow(unused)]
mod messages;

use std::{net::TcpStream, unimplemented};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use vevgren_api_interface::{commands::ApiCommand, events::ApiEvent};

const APIURL: &str = "wss://ws.kraken.com";

/// Kraken API uses the vevgren_api_interface to standardize messages,.
pub struct Api {
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    connected: bool,
}

impl Api {
    pub fn new() -> Self {
        Self {
            socket: None,
            connected: false,
        }
    }

    fn connect_to_socket(&mut self) -> Result<(), String> {
        match connect(APIURL) {
            Ok(x) => {
                self.socket = Some(x.0);
                self.connected = true;
            }
            Err(e) => {
                println!("error when connecting to Kraken APi: \n{}", e);
                return Err(e.to_string());
            }
        };
        Ok(())
    }

    fn subscribe(&mut self, symbol: String) {
        match self.socket.as_mut() {
            Some(x) => {
                x.write_message(tungstenite::Message::Text("Hejsan".to_string()));
            }
            None => {
                println!("socket not connected, cant subscribe");
            }
        };
    }

    fn read_message(&self) -> Message {
        // Implements a future to be able to listen to both command and events
        unimplemented!()
    }

    ///Returns the ApiEvent of a specific message to be sent for handling
    fn parse_to_event(message: String) -> ApiEvent {
        unimplemented!()
    }

    ///Make a Message that can be directly sent over socket.
    fn parse_to_message() {
        // takes a command? and depending on action should do stuff
        unimplemented!()
    }
}
