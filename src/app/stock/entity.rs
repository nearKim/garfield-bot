use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    pub ticker: String,
    pub close: f32,
    pub high: f32,
    pub low: f32,
}
