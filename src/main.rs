use std::iter::once;
use crate::app::stock::entity::StockData;
use crate::app::stock::repository::StockRepository;
use crate::app::stock::service::StockService;
use crate::app::weather::repository::AccuWeatherRepository;
use crate::app::weather::service::WeatherService;

mod app;

#[tokio::main]
async fn main() {
    // let repository = AccuWeatherRepository {};
    // let service = WeatherService { repository };
    // let data = service.get_today_weather_data().await;
    // let p_data = service.get_today_particulate_data().await;
    // println!("{}", service.create_weather_msg(&data));
    // println!("{}", service.create_particulate_msg(&p_data));

    let repository = StockRepository {};
    let s_service = StockService { repository };
    let data = s_service.get_yesterday_stock_data().await;
    match data {
        Ok(p) => {
            let msg = s_service.create_stock_msg(vec![p.0, p.1]);
            println!("{}", msg);
        },
        Err(s) => println!("에러메시지 보여줘야 함")
    }
}
