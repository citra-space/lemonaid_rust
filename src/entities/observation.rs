use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpticalObservation {
    pub id: Option<String>,
    pub satellite_id: String,
    pub epoch: DateTime<Utc>,
    pub right_ascension_deg: f64,
    pub declination_deg: f64,
    pub right_ascension_rate: Option<f64>,
    pub declination_rate: Option<f64>,
    pub magnitude: Option<f64>,
    pub sensor_latitude_deg: f64,
    pub sensor_longitude_deg: f64,
    pub sensor_altitude_km: f64,
    pub telescope_id: Option<String>,
    pub upload_id: Option<String>,
    pub user_id: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateOpticalObservationRequest {
    pub satellite_id: String,
    pub epoch: DateTime<Utc>,
    pub right_ascension_deg: f64,
    pub declination_deg: f64,
    pub right_ascension_rate: Option<f64>,
    pub declination_rate: Option<f64>,
    pub magnitude: Option<f64>,
    pub sensor_latitude_deg: f64,
    pub sensor_longitude_deg: f64,
    pub sensor_altitude_km: f64,
    pub telescope_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationCount {
    pub hour: DateTime<Utc>,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationQuery {
    pub satellite_id: Option<String>,
    pub telescope_id: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
