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

fn funding_rate() -> f32 {
    let url = reqwest::Url::parse_with_params(
        "https://api.bybit.com/v5/market/tickers",
        &[("category", "inverse"), ("symbol", "BTCUSD")]
    ).expect("Can't generate URL");

    let res = reqwest::blocking::get(url).expect("Failed to GET url");
    let payload: Payload = serde_json::from_reader(res).expect("Failed to parse response");

    if payload.result.list.len() != 1 {
        eprintln!("Expected a single element");
    }

    payload.result.list[0].funding_rate.parse().expect("Failed to parse funding rate")
}

fn main() {
    let funding = funding_rate();
    println!("{:?}%", funding * 100.0);

}
