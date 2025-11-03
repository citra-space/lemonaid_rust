use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Telescope {
    pub id: String,
    pub name: String,
    #[serde(rename = "groundStationId")]
    pub groundstation_id: Option<String>,
    pub user_id: String,
    pub satellite_id: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "lastConnectionEpoch")]
    pub last_connected_at: Option<DateTime<Utc>>,
    #[serde(rename = "angularNoise")]
    pub angular_noise_arcsec: f64,
    #[serde(rename = "fieldOfView")]
    pub field_of_view_deg: f64,
    #[serde(rename = "maxMagnitude")]
    pub limiting_magnitude: f64,
    #[serde(rename = "minElevation")]
    pub min_elevation_deg: f64,
    #[serde(rename = "maxSlewRate")]
    pub max_slew_rate_deg_per_sec: f64,
    #[serde(rename = "homeAzimuth")]
    pub home_azimuth_deg: f64,
    #[serde(rename = "homeElevation")]
    pub home_elevation_deg: f64,
    pub automated_scheduling: bool
}