use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub min_temp: f32,
    pub max_temp: f32,
    pub date_time: String,
    pub has_precipitation: bool
}

impl WeatherData {
    pub fn new(min_temp: f32, max_temp: f32, date_time: String, has_precipitation: bool) -> Self {
        Self {
            min_temp,
            max_temp,
            date_time,
            has_precipitation
        }
    }
}