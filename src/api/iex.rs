extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize};

use super::base::{Client, Quote};

// TODO(aksiksi): Use Option for any potentially missing data
// Let the user decide whether to check or panic
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct IEXQuote {
    symbol: String,
    companyName: String,
    open: Option<f32>,
    openTime: Option<u64>,
    close: f32,
    closeTime: u64,
    high: Option<f32>,
    low: Option<f32>,
    latestPrice: f32,
    latestVolume: u64,
    marketCap: u64,
    peRatio: f32,
    week52High: f32,
    week52Low: f32,
    ytdChange: f32,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct IEXNews {
    headline: String,
    source: String,
    summary: String,
    lang: String,
    datetime: u64,
}

#[derive(Debug)]
pub struct IEXClient {
    pub api_key: String,
    endpoint: &'static str,
    client: reqwest::Client,
}

impl IEXClient {
    pub fn get_news(&self, symbol: &str) ->
        Result<Vec<IEXNews>, Box<std::error::Error>> {
        let request_url =
            format!("{0}/stock/{1}/news/last/10?token={2}", self.endpoint,
                    symbol, self.api_key);
        let resp: String = self.request(request_url.as_str());

        let news: Vec<IEXNews> = serde_json::from_str(&resp)
            .expect("IEX News parsing failed");

        Ok(news)
    }
}

impl Client for IEXClient {
    fn new(api_key: &str) -> Self {
        // TODO(aksiksi): Verify API key here
        // Or we can add a "verify" method
        IEXClient {
            api_key: api_key.to_string(),
            endpoint: "https://cloud.iexapis.com/stable/",
            client: reqwest::Client::new(),
        }
    }

    // TODO: Clean this up?
    fn request(&self, request_url: &str) -> String {
        // Unwrap the response here because request failure is fatal
        let resp = self.client.get(request_url)
            .send().expect("Request failed")
            .text().expect("Response body parsing failed");
        resp
    }

    fn get_quote(&self, symbol: &str) -> Result<Quote, Box<std::error::Error>> {
        // Get info from API as JSON
        let request_url =
            format!("{0}/stock/{1}/quote?token={2}", self.endpoint, symbol,
                    self.api_key);

        let resp: String = self.request(request_url.as_str());

        // Parse JSON into struct
        let quote: IEXQuote = serde_json::from_str(&resp)
            .expect("IEX Quote parsing failed");

        // Build generic Quote struct
        let quote = Quote {
            symbol: quote.symbol,
            company_name: quote.companyName,
            open: quote.open.unwrap_or(-1.0),
            open_time: quote.openTime.unwrap_or(0),
            close: quote.close,
            close_time: quote.closeTime,
            high: quote.high.unwrap_or(-1.0),
            low: quote.low.unwrap_or(-1.0),
            latest: quote.latestPrice,
            latest_volume: quote.latestVolume,
            market_cap: quote.marketCap,
            pe_ratio: quote.peRatio,
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
