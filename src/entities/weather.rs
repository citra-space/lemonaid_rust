use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub units: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherCondition {
    pub id: Option<i64>,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub dt: Option<i64>,
    pub sunrise: Option<i64>,
    pub sunset: Option<i64>,
    pub temp: f64,
    pub feels_like: Option<f64>,
    pub pressure: Option<i64>,
    pub humidity: Option<i64>,
    pub dew_point: Option<f64>,
    pub uvi: Option<f64>,
    pub clouds: Option<i64>,
    pub visibility: Option<i64>,
    pub wind_speed: Option<f64>,
    pub wind_deg: Option<i64>,
    pub wind_gust: Option<f64>,
    pub weather: Option<Vec<WeatherCondition>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinutelyForecast {
    pub dt: i64,
    pub precipitation: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: Option<String>,
    pub timezone_offset: Option<i64>,
    pub current: CurrentWeather,
    pub minutely: Option<Vec<MinutelyForecast>>,
}
