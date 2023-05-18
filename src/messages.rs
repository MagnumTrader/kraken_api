use serde::Serialize;

#[derive(Debug, Serialize)]
struct SubscribeMessage {
    symbols: Vec<String>,
}
