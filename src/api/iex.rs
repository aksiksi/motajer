extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize};

use super::base::{Client, Quote};

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct IEXQuoteAPI {
    symbol: String,
    companyName: String,
    open: f32,
    openTime: u64,
    close: f32,
    closeTime: u64,
    high: f32,
    low: f32,
    latestPrice: f32,
    latestVolume: u64,
    marketCap: u64,
    week52High: f32,
    week52Low: f32,
    ytdChange: f32,
}

#[derive(Debug)]
pub struct IEXClient {
    pub api_key: String,
    endpoint: &'static str,
    client: reqwest::Client,
}

impl Client for IEXClient {
    fn new(api_key: &str) -> Self {
        IEXClient {
            api_key: api_key.to_string(),
            endpoint: "https://cloud.iexapis.com/stable/",
            client: reqwest::Client::new(),
        }
    }

    fn get_quote(&self, symbol: &str) -> Result<Quote, Box<std::error::Error>> {
        // Get info from API as JSON
        let request_url =
            format!("{0}/stock/{1}/quote?token={2}", self.endpoint, symbol,
                    self.api_key);
        let resp = self.client.get(request_url.as_str()).send()?.text()?;

        // Parse JSON into struct
        let quote: IEXQuoteAPI = serde_json::from_str(&resp)?;

        // Build generic Quote struct
        let quote = Quote {
            symbol: quote.symbol,
            company_name: quote.companyName,
            open: quote.open,
            open_time: quote.openTime,
            close: quote.close,
            close_time: quote.closeTime,
            high: quote.high,
            low: quote.low,
            latest: quote.latestPrice,
            latest_volume: quote.latestVolume,
            market_cap: quote.marketCap,
            high_yearly: quote.week52High,
            low_yearly: quote.week52Low,
            ytd_change: quote.ytdChange,
        };

        Ok(quote)
    }

    fn get_price(&self, symbol: &str) -> Result<f32, Box<std::error::Error>> {
        Ok(self.get_quote(symbol)?.latest)
    }
}
