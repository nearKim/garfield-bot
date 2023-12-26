use serde::Deserialize;
use crate::app::weather::entity::WeatherData;

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