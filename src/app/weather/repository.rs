use reqwest::{get, Error};
use crate::app::weather::entity::{ParticulateData, WeatherData};
use crate::app::weather::json::{ParticulateResult, WeatherResult};

pub struct AccuWeatherRepository;
impl AccuWeatherRepository {
    pub async fn get(&self) -> WeatherData {
        let url = "http://dataservice.accuweather.com/forecasts/v1/daily/1day/226081?apikey=lYV4i76QL6ykO9VKsMSEI4UbsxzosH2H&metric=true";
        let response = get(url).await.unwrap();
        let result: WeatherResult = response.json().await.unwrap();

        result.to_domain()
    }
    pub async fn get_particulate(&self) -> ParticulateData {
        let url = "http://openAPI.seoul.go.kr:8088/sample/json/RealtimeCityAir/1/5/동남권/강남구";
        let response = get(url).await.unwrap();
        let result: ParticulateResult = response.json().await.unwrap();

        result.to_domain()
    }
}