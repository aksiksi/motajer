#[derive(Debug)]
pub struct Quote {
    pub symbol: String,
    pub company_name: String,
    pub open: f32,
    pub open_time: u64,
    pub close: f32,
    pub close_time: u64,
    pub high: f32,
    pub low: f32,
    pub latest: f32,
    pub latest_volume: u64,
    pub market_cap: u64,
    pub high_yearly: f32,
    pub low_yearly: f32,
    pub ytd_change: f32,
}

/// Defines a generic stocks API client
pub trait Client {
    fn new(api_key: &str) -> Self;
    fn get_quote(&self, symbol: &str) -> Result<Quote, Box<std::error::Error>>;
    fn get_price(&self, symbol: &str) -> Result<f32, Box<std::error::Error>>;
}
