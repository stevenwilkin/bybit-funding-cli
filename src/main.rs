use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Payload {
    result: Result,
}

#[derive(Deserialize,Debug)]
struct Result {
    list: Vec<Ticker>,
}

#[derive(Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    #[serde(default)]
    funding_rate: String,
}

struct FundingRate {
    client: reqwest::blocking::Client,
    url: reqwest::Url,
}

impl FundingRate {
    fn new() -> Self {
        let url = reqwest::Url::parse_with_params(
            "https://api.bybit.com/v5/market/tickers",
            &[("category", "inverse"), ("symbol", "BTCUSD")]
        ).expect("Can't generate URL");

        Self {
            client: reqwest::blocking::Client::new(),
            url: url,
        }
    }

    fn get(&self) -> f32 {
        let res = self.client.get(self.url.as_ref()).send().expect("Failed to GET url");
        let payload: Payload = serde_json::from_reader(res).expect("Failed to parse response");

        if payload.result.list.len() != 1 {
            panic!("Expected a single element");
        }

        payload.result.list[0].funding_rate.parse().expect("Failed to parse funding rate")
    }
}

fn main() {
    let funding_rate = FundingRate::new();
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        loop {
            tx.send(1).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        rx.recv().unwrap();
        let funding = funding_rate.get();
        println!("\x1b[2J\x1b[H\x1b[?25l");   // clear screen, move cursor to top of screen, hide cursor
        println!("  {:?}%", funding * 100.0);
    }
}
