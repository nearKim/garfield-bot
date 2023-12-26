use std::rc::Rc;
use crate::app::weather::entity::WeatherData;
use crate::app::weather::repository::AccuWeatherRepository;

pub struct WeatherService {
    pub repository: AccuWeatherRepository,
}

impl WeatherService {
    pub fn new(repository: AccuWeatherRepository) -> Self {
        Self { repository }
    }

    pub async fn get_today_weather_data(&self) -> WeatherData {
        self.repository.get().await.expect("Failed to get WeatherData")
    }

    pub fn create_weather_msg(&self, data: &WeatherData) -> String {
        format!("[{}의 날씨]\n * 최고기온: {}°C\n* 최저기온: {}°C\n눈 또는 비: {}",
                data.date_time,
                data.max_temp,
                data.min_temp,
                data.has_precipitation
        )
    }
}