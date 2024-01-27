use reqwest::Error;
use serde::Serialize;
use crate::app::stock::repository::StockRepository;
use crate::app::stock::service::StockService;
use crate::app::weather::repository::AccuWeatherRepository;
use crate::app::weather::service::WeatherService;
use tokio::join;

mod app;

async fn weather_msg() -> String {
    let repository = AccuWeatherRepository {};
    let service = WeatherService { repository };
    let data = service.get_today_weather_data().await;
    let p_data = service.get_today_particulate_data().await;
    let msg = service.create_weather_msg(&data) + &service.create_particulate_msg(&p_data);
    msg
}

async fn stock_msg() -> String {
    let repository = StockRepository {};
    let s_service = StockService { repository };
    let data = s_service.get_yesterday_stock_data().await;
    match data {
        Ok(p) => s_service.create_stock_msg(vec![p.0, p.1]),
        Err(s) => s,
    }
}


#[derive(Serialize, Debug)]
struct Text {
    text: String
}


async fn send_message_to_slack(webhook_url: &str, message: &str) -> Result<(), String> {
    let payload = Text { text : message.to_string() };
    let client = reqwest::Client::new();
    let response = client.post(webhook_url)
        .json(&payload)
        .send()
        .await.expect("shit!");

    // Check if the request was successful (HTTP status code 200 OK)
    if response.status().is_success() {
        println!("Message sent successfully!");
        Ok(())
    } else {
        println!("Failed to send message. Status code: {}", response.status());
        Err("fuck!".to_string())

    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let result = join!(weather_msg(), stock_msg());
    let msg = result.0 + &result.1;

    send_message_to_slack("", &msg).await
}
