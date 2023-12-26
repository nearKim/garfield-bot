use std::rc::Rc;
use crate::app::weather::repository::AccuWeatherRepository;
use crate::app::weather::service::WeatherService;

mod app;

#[tokio::main]
async fn main() {
    let repository = AccuWeatherRepository {};
    let service = WeatherService::new(repository);
    let data = service.get_today_weather_data().await;
    println!("{:?}", data);
}
