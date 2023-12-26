use std::str::FromStr;
use tokio::try_join;
use crate::app::stock::entity::StockData;
use crate::app::stock::repository::StockRepository;

pub struct StockService {
    pub repository: StockRepository
}

impl StockService {
    pub async fn get_yesterday_stock_data(&self) ->  Result<(StockData, StockData), String> {
        try_join!(self.repository.get("AAPL"), self.repository.get("TSLA"))
    }

    pub fn create_stock_msg(&self, stock_data_vec: Vec<StockData>) -> String {
        let mut result = String::from_str("[주식 정보]\n").expect("Fail");

        for data in stock_data_vec {
            result.push_str(format!("<종목: {}>\n", data.ticker).as_str());
            result.push_str(format!("종가: {}\n", data.close).as_str());
            result.push_str(format!("상한가: {}\n", data.high).as_str());
            result.push_str(format!("하한가: {}\n", data.low).as_str());
        }
        result
    }
}