use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub min_temp: f32,
    pub max_temp: f32,
    pub date_time: String,
    pub has_precipitation: bool,
}

#[derive(Deserialize, Debug)]
pub struct ParticulateData {
    pub pm10: f32,
    pub pm25: f32,
}
