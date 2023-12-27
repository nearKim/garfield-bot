use std::rc::Rc;
use crate::app::weather::entity::{ParticulateData, WeatherData};
use crate::app::weather::repository::AccuWeatherRepository;

pub struct WeatherService {
    pub repository: AccuWeatherRepository,
}

impl WeatherService {
    pub async fn get_today_weather_data(&self) -> WeatherData {
        self.repository.get().await
    }

    pub async fn get_today_particulate_data(&self) -> ParticulateData {
        self.repository.get_particulate().await
    }

    pub fn create_weather_msg(&self, data: &WeatherData) -> String {
        format!("[{}]\n* 최고/최저: {} / {}\n* 눈비: {}\n",
                data.date_time,
                data.max_temp,
                data.min_temp,
                data.has_precipitation
        )
    }

    pub fn create_particulate_msg(&self, data: &ParticulateData) -> String {
        let pm10_criteria = match data.pm10 {
            x if x <= 30.0 => "좋음",
            x if x <= 80.0 => "보통",
            x if x <= 150.0 => "나쁨",
            _ => "매우나쁨"
        };
        let pm25_criteria = match data.pm25 {
            x if x <= 15.0 => "좋음",
            x if x <= 35.0 => "보통",
            x if x <= 75.0 => "나쁨",
            _ => "매우나쁨"
        };
        format!("* 미세/초미세: {} / {}\n", pm10_criteria, pm25_criteria)
    }
}