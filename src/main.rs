use crate::app::stock::repository::StockRepository;
use crate::app::stock::service::StockService;
use crate::app::weather::repository::AccuWeatherRepository;
use crate::app::weather::service::WeatherService;
use crate::json::SlackHook;
use json::Text;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use tokio::join;

mod app;
mod json;

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

async fn send_message_to_slack(webhook_url: &str, message: &str) -> Result<(), String> {
    let payload = Text {
        text: message.to_string(),
    };
    let client = reqwest::Client::new();
    let response = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Message sent successfully!");
        Ok(())
    } else {
        println!("Failed to send message. Status code: {}", response.status());
        Err("Failed to send message".to_string())
    }
}

fn read_json_bytes(json_bytes: &[u8]) -> SlackHook {
    let json_str = std::str::from_utf8(json_bytes).unwrap();
    let json_data: SlackHook = serde_json::from_str(json_str).unwrap();
    json_data
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let result = join!(weather_msg(), stock_msg());
    let msg1 = &result.0;
    let msg2 = result.0 + &result.1;
    let json_bytes = include_bytes!("secret.json");
    let slack_hook = read_json_bytes(json_bytes);

    send_message_to_slack(&slack_hook.ml_chat, &msg1).await;
    send_message_to_slack(&slack_hook.garfield_random, &msg2).await
}
