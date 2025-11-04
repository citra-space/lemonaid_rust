use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Antenna {
    pub id: String,
    pub user_id: String,
    pub user_group_id: Option<String>,
    #[serde(rename = "groundStationId")]
    pub groundstation_id: Option<String>,
    pub satellite_id: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "lastConnectionEpoch")]
    pub last_connected_at: Option<DateTime<Utc>>,
    pub name: String,
    #[serde(rename = "minFrequency")]
    pub min_frequency_hz: f64,
    #[serde(rename = "maxFrequency")]
    pub max_frequency_hz: f64,
    #[serde(rename = "minElevation")]
    pub min_elevation_deg: f64,
    #[serde(rename = "maxSlewRate")]
    pub max_slew_rate_deg_per_sec: f64,
    #[serde(rename = "homeAzimuth")]
    pub home_azimuth_deg: f64,
    #[serde(rename = "homeElevation")]
    pub home_elevation_deg: f64,
    #[serde(rename = "halfPowerBeamWidth")]
    pub half_power_beam_width_deg: f64
}