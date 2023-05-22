use serde::Serialize;
use std::vec;

#[derive(Debug, Serialize)]
pub struct Subscription<'a> {
    event: SubscriptionEvent,
    pair: Vec<&'a str>,
    subscription: SubscriptionType,
}
impl<'a> Subscription<'a> {
    pub fn new(symbol: &'a str, sub_name: SubscriptionName) -> Self {
        Self {
            event: SubscriptionEvent::Subscribe,
            pair: vec![symbol],
            subscription: SubscriptionType {
                name: sub_name,
                depth: None,
                token: None,
            },
        }
    }

    //TODO: Builder pattern to easily subscribe to different types, also construct the correct
    //message based on the functions called, can also return error if for example another trade
    //type is already selected.
}
#[allow(unused)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionEvent {
    Subscribe,
    Unsubsribe,
}

#[derive(Debug, Serialize)]
struct SubscriptionType {
    name: SubscriptionName,
    ///setting for depth when SubscriptionName == book
    ///
    ///Valid Options are: 10, 25, 100, 500, 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionName {
    Trade,
    Book,
    Ticker,
    Ohlc,
    Spread,
    #[serde(rename(serialize = "ownTrades"))]
    OwnTrades,
    #[serde(rename(serialize = "ownOrders"))]
    OwnOrders,
}

#[allow(unused)]
#[derive(Debug, Serialize)]
enum OhlcInterval {
    One,
    Five,
    Fifteen,
    Thirty,
    Hour,
    Daily,
}
