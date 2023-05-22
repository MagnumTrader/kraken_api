mod messages;

use messages::{Subscription, SubscriptionName};
use serde_json::{to_string, Value};
use std::{net::TcpStream, str::FromStr};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use vevgren_api_interface::{
    commands::ApiCommand,
    events::{ApiEvent, MarketUpdate, PriceUpdate, Trade},
};

// URL and settings for this Api
const APIURL: &str = "wss://ws.kraken.com";
//
//
//
//
//
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

    pub fn connect_to_socket(&mut self) -> Result<(), String> {
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
    pub fn subscribe(&mut self, symbol: &str) {
        match self.socket.as_mut() {
            Some(x) => {
                let subs_msg = Subscription::new(symbol, SubscriptionName::Trade); //TODO: Builder?
                let s = to_string(&subs_msg).unwrap();
                x.write_message(tungstenite::Message::Text(s)).unwrap();
            }
            None => {
                println!("socket not connected, can't subscribe");
            }
        };
    }

    pub async fn read_message(&mut self) -> Message {
        //BUG: Behöver en yield här! så vi inte behöver vänta på HB
        self.socket.as_mut().unwrap().read_message().unwrap()
    }

    ///Returns the ApiEvent of a specific message to be sent for handling
    pub fn parse_to_event(&self, s: String) -> ApiEvent {
        let message_as_value = Value::from_str(&s).unwrap();

        match message_as_value {
            Value::Array(x) => {
                //HACK For showing the rows, remove later.
                for row in x.iter() {
                    println!("{}", row);
                }
                // Wierd that Kraken API sends Array or Object..
                // destructuring array, what type of message is this?
                if let Value::String(y) = &x[2] {
                    match y.as_str() {
                        "trade" => {
                            let api_channel = x[0].as_u64().unwrap();
                            let symbol = x[3].as_str().unwrap().to_string();
                            let trades = x[1].as_array().unwrap().to_vec();

                            return new_trade_event(api_channel, symbol, trades);
                        }
                        "book" => todo!(), // initial snapshot then updates
                        "spread" => todo!(),
                        _ => {
                            println!("Unknown Event!");
                        }
                    }
                    return ApiEvent::Error("Unknown Message".to_string());
                } else {
                    return ApiEvent::Error("unknown Message".to_string());
                }
            }
            Value::Object(x) => {
                // Strings for matching the event

                // get the event of the message.

                if let Some(event) = x.get("event") {
                    match event {
                        Value::String(event_string) => {
                            match event_string.as_str() {
                                "systemStatus" => {
                                    //TODO Systemsstatus, vad finns det mer än connected?
                                }
                                "subscriptionStatus" => {
                                    //TODO Att göra, confirmation att vi har subscribeat
                                    //status string :	online|maintenance|cancel_only|limit_only|post_only
                                }
                                "maintenance" => {
                                    // FIXME Nytt meddelande.
                                    return ApiEvent::Error("Maintan".to_string());
                                }
                                "heartbeat" => return ApiEvent::Heartbeat,
                                _ => {
                                    println!("Error, unknow event!");
                                    return ApiEvent::Error("Error".to_string());
                                }
                            }
                        }
                        _ => {
                            println!("Error: Event not a string!!");
                            return ApiEvent::Error("Error".to_string());
                        }
                    }
                } else {
                    println!("Could not parse event");
                    return ApiEvent::Error("Error".to_string());
                };
                //Last resort cant parse
                ApiEvent::Error("Error, cant parse Value".to_string())
            }
            _ => ApiEvent::Error(" unknown Value from API".to_string()),
        }
    }
}

fn new_trade_event(_api_channel: u64, symbol: String, trades: Vec<Value>) -> ApiEvent {
    let collected_trades: Vec<Trade> = trades
        .into_iter()
        .map(|x| {
            let timestamp = 0;
            let price = x[0].as_str().unwrap().parse().unwrap();
            let volume = x[1].as_str().unwrap().parse().unwrap();
            // let time = x[2].as_str().unwrap().chars().collect();
            let side = x[3].as_str().unwrap().chars().next().unwrap();
            let order_type = x[4].as_str().unwrap().chars().next().unwrap();

            Trade {
                timestamp,
                price,
                volume,
                side,
                order_type,
            }
        })
        .collect();

    ApiEvent::Market(MarketUpdate::Price(PriceUpdate {
        symbol,
        trades: collected_trades,
    }))
}
