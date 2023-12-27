use std::env;
use aws_config::{BehaviorVersion, Region};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sns::{Client, Error};
use tokio::join;
use crate::app::stock::repository::StockRepository;
use crate::app::stock::service::StockService;
use crate::app::weather::repository::AccuWeatherRepository;
use crate::app::weather::service::WeatherService;

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
        Err(s) => "어제 장 데이터를 가져오지 못하였습니다.".to_string()
    }

}
async fn subscribe_and_publish(
    client: &Client,
    topic_arn: &str,
    phone: &str,
    msg: String,
) -> Result<(), Error> {
    println!("Receiving on topic with ARN: `{}`", topic_arn);

    let rsp = client
        .subscribe()
        .topic_arn(topic_arn)
        .protocol("sms")
        .endpoint(phone)
        .send()
        .await?;

    println!("Added a subscription: {:?}", rsp);

    let rsp = client
        .publish()
        .topic_arn(topic_arn)
        .message(msg)
        .send()
        .await?;

    println!("Published message: {:?}", rsp);

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error> {

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::from_static("ap-northeast-1"))
        .load()
        .await;

    let client = Client::new(&config);

    let result = join!(weather_msg(), stock_msg());
    let msg = result.0 + &result.1;

    subscribe_and_publish(
        &client,
        "arn:aws:sns:ap-northeast-1:764123066160:garfield-bot-topic",
        "+821071550868",
        msg
    ).await
}
