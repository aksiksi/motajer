use std::env;

mod api;

use api::iex;
use api::base::Client;

fn main() {
    let symbol = "AAPL";
    let api_key = env::var("IEXCLOUD_API_KEY").unwrap();
    let iex_client = iex::IEXClient::new(api_key.as_str());

    println!("{:#?}", iex_client);

    println!("Latest stock data for {0}: ${1:?}", symbol,
             iex_client.get_quote(symbol).unwrap());
}
