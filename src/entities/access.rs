use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteAccessToGroundstationRequest {
    #[serde(rename = "groundStationId")]
    pub groundstation_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(rename = "minElevation")]
    pub min_elevation_deg: f64,
    #[serde(rename = "minDuration")]
    pub min_duration_minutes: f64,
    pub min_frequency_mhz: Option<f64>,
    pub max_frequency_mhz: Option<f64>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackingParameters {
    #[serde(rename = "epoch")]
    pub time: DateTime<Utc>,
    #[serde(rename = "azimuth")]
    pub azimuth_deg: f64,
    #[serde(rename = "elevation")]
    pub elevation_deg: f64,
    #[serde(rename = "azimuthRate")]
    pub azimuth_rate_deg_s: Option<f64>,
    #[serde(rename = "elevationRate")]
    pub elevation_rate_deg_s: Option<f64>,
    #[serde(rename = "range")]
    pub range_km: Option<f64>,
    #[serde(rename = "rangeRate")]
    pub range_rate_km_s: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HorizonAccess {
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    #[serde(rename = "groundStationId")]
    pub groundstation_id: String,
    #[serde(rename = "groundStationName")]
    pub groundstation_name: Option<String>,
    pub start: TrackingParameters,
    pub end: TrackingParameters,
    #[serde(rename = "duration")]
    pub duration_minutes: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SensorFrame {
    TEME,
    J2000
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FOVAccessRequest {
    #[serde(rename = "epoch")]
    pub time: DateTime<Utc>,
    #[serde(rename = "rightAscension")]
    pub right_ascension_deg: f64,
    #[serde(rename = "declination")]
    pub declination_deg: f64,
    #[serde(rename = "fieldOfView")]
    pub field_of_view_deg: f64,
    #[serde(rename = "sensorLatitude")]
    pub sensor_latitude_deg: f64,
    #[serde(rename = "sensorLongitude")]
    pub sensor_longitude_deg: f64,
    #[serde(rename = "sensorAltitude")]
    pub sensor_altitude_km: f64,
    pub sensor_frame: SensorFrame
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FOVAccessResponse {
    pub satellite_id: String,
    #[serde(rename = "name")]
    pub satellite_name: Option<String>,
    #[serde(rename = "rightAscension")]
    pub right_ascension_deg: f64,
    #[serde(rename = "declination")]
    pub declination_deg: f64
}
