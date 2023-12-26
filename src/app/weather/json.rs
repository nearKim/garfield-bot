use std::collections::HashMap;
use serde::Deserialize;
use crate::app::weather::entity::{ParticulateData, WeatherData};

#[derive(Deserialize, Debug)]
struct TempData {
    Value: f32,
    Unit: String,
    UnitType: u8,
}

#[derive(Deserialize, Debug)]
struct Temperature {
    Minimum: TempData,
    Maximum: TempData,
}

#[derive(Deserialize, Debug)]
struct General {
    Icon: u8,
    IconPhrase: String,
    HasPrecipitation: bool,
}

#[derive(Deserialize, Debug)]
struct DailyForecast {
    Date: String,
    EpochDate: u32,
    Temperature: Temperature,
    Day: General,
    Night: General,
    Sources: Vec<String>,
    MobileLink: String,
    Link: String,
}

#[derive(Deserialize, Debug)]
struct Headline {
    EffectiveDate: String,
    EffectiveEpochDate: u32,
    Severity: u8,
    Text: String,
    Category: String,
    EndDate: String,
    EndEpochDate: u32,
    MobileLink: String,
    Link: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResult {
    Headline: Headline,
    pub DailyForecasts: Vec<DailyForecast>,
}

impl WeatherResult {
    pub fn to_domain(&self) -> WeatherData {
        let forecast = self.DailyForecasts.first().unwrap();
        WeatherData {
            min_temp : forecast.Temperature.Minimum.Value,
            max_temp : forecast.Temperature.Maximum.Value,
            date_time : forecast.Date
                            .split('T')
                            .next()
                            .unwrap_or("[이상한 데이터]")
                            .to_string(),
            has_precipitation:  forecast.Day.HasPrecipitation || forecast.Night.HasPrecipitation
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ParticulateInfo {
    MSRDT: String,
    MSRRGN_NM: String,
    MSRSTE_NM: String,
    PM10: f32,
    PM25: f32,
    O3: f32,
    NO2: f32,
    CO: f32,
    SO2: f32,
    IDEX_NM: String,
    IDEX_MVL: f32,
    ARPLT_MAIN: String
}

#[derive(Deserialize, Debug)]
pub struct ApiResult {
    list_total_count: u8,
    RESULT: HashMap<String, String>,
    row: Vec<ParticulateInfo>

}
#[derive(Deserialize, Debug)]
pub struct ParticulateResult {
    RealtimeCityAir: ApiResult
}

impl ParticulateResult {
    pub fn to_domain(&self) -> ParticulateData {
        let data =self.RealtimeCityAir.row.first().unwrap();

        ParticulateData {
            pm10: data.PM10,
            pm25: data.PM25
        }
    }
}
