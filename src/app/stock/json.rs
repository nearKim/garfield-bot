use crate::app::stock::entity::StockData;
use chrono::{Datelike, Duration, Local};
use chrono_tz::Asia::Seoul;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    pub output_size: String,
    #[serde(rename = "5. Time Zone")]
    pub time_zone: String,
}

#[derive(Deserialize, Debug)]
struct DailyData {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}
#[derive(Deserialize, Debug)]
pub struct StockResult {
    #[serde(rename = "Meta Data")]
    metadata: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    daily_data: HashMap<String, DailyData>,
}

impl StockResult {
    pub fn to_domain(&self, ticker: &str) -> Result<StockData, String> {
        let yesterday = Local::now().with_timezone(&Seoul) - Duration::days(1);
        let key_string = format!(
            "{}-{}-{}",
            yesterday.year(),
            yesterday.month(),
            yesterday.day()
        );
        let key = key_string.as_str();
        let data = self
            .daily_data
            .get(key);

        match data {
            Some(d) => Ok(StockData {
                ticker: ticker.to_string(),
                close: d.close.parse().unwrap(),
                high: d.high.parse().unwrap(),
                low: d.low.parse().unwrap(),
            }),
            None => Err("No Stock Data for yesterday".to_string())
        }
    }
}
