use std::env;

mod api;

use api::iex;
use api::base::Client;

fn main() {
    let symbol = "AAPL";
    let api_key = env::var("IEXCLOUD_API_KEY");
    let iex_client = match api_key {
        Ok(v) => iex::IEXClient::new(v.as_str()),
        Err(_) => {
            println!("No API key found!");
            return ();
        }
    };

    println!("{:#?}", iex_client);

    let quote = match iex_client.get_quote(symbol) {
        Ok(v) => v,
        Err(e) => panic!("Error: {:?}", e)
    };

    println!("Latest stock data for {0}: ${1:?}", symbol, quote);

    let news = match iex_client.get_news(symbol) {
        Ok(v) => v,
        Err(e) => panic!("Error while fetching news: {:?}", e)
    };

    println!("Latest news for {0}: ${1:?}", symbol, news);
}
