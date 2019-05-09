use std::env;

mod stocks;

fn main() {
    let symbol = "AAPL";
    let api_key = env::var("ALPHAVANTAGE_API_KEY").unwrap();
    let s = stocks::Stock {
        ticker: symbol.to_string(),
        api_key: api_key.clone(),
    };
    println!("{:#?}", s);

    println!("Latest price for {0}: ${1}", symbol, s.get_price().unwrap());
}
