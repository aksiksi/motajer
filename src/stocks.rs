extern crate reqwest;
extern crate serde_json;

static ALPHAVANTAGE_QUOTE_API: &str =
    "https://www.alphavantage.co/query?function=GLOBAL_QUOTE";

#[derive(Debug)]
pub struct Stock {
    pub ticker: String,
    pub api_key: String,
}

impl Stock {
    // TODO(aksiksi): Create a Client and re-use it for every API call (?)
    pub fn get_price(&self) -> Result<f32, Box<std::error::Error>> {
        // Build API request URL
        let request_url =
            format!("{0}&symbol={1}&apikey={2}", ALPHAVANTAGE_QUOTE_API, self.ticker.as_str(), self.api_key.as_str());
        println!("{}", request_url);

        // JSON API response as text
        let text: String = reqwest::get(request_url.as_str())?.text()?;

        // Serialize into JSON
        // TODO(aksiksi): Use the strongly-typed approach for better error checking
        let json: serde_json::Value = serde_json::from_str(text.as_str())?;
        let current_price: f32 = json["Global Quote"]["05. price"].as_str()
            .unwrap().parse().unwrap();

        return Ok(current_price);
    }
}
