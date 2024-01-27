use crate::app::stock::entity::StockData;
use crate::app::stock::json::StockResult;
use reqwest::{get, Error};

pub struct StockRepository;
impl StockRepository {
    pub async fn get(&self, symbol: &str) -> Result<StockData, String> {
        let url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey=QS6LJGHKPW7C5GLD", symbol);
        let response = get(url).await.unwrap();
        let result: StockResult = response.json().await.unwrap();

        result.to_domain(&symbol)
    }
}
