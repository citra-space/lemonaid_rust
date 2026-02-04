use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Telescope {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "groundStationId")]
    pub groundstation_id: Option<String>,
    pub user_id: Option<String>,
    pub user_group_id: Option<String>,
    pub username: Option<String>,
    pub satellite_id: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "lastConnectionEpoch")]
    pub last_connected_at: Option<DateTime<Utc>>,
    #[serde(rename = "angularNoise")]
    pub angular_noise_arcsec: Option<f64>,
    #[serde(rename = "fieldOfView")]
    pub field_of_view_deg: Option<Value>,
    #[serde(rename = "legacyFieldOfView")]
    pub legacy_field_of_view_deg: Option<f64>,
    #[serde(rename = "maxMagnitude")]
    pub limiting_magnitude: Option<f64>,
    #[serde(rename = "minElevation")]
    pub min_elevation_deg: Option<f64>,
    #[serde(rename = "maxSlewRate")]
    pub max_slew_rate_deg_per_sec: Option<f64>,
    #[serde(rename = "homeAzimuth")]
    pub home_azimuth_deg: Option<f64>,
    #[serde(rename = "homeElevation")]
    pub home_elevation_deg: Option<f64>,
    pub automated_scheduling: Option<bool>,
    pub horizontal_pixel_count: Option<i64>,
    pub vertical_pixel_count: Option<i64>,
    pub pixel_size: Option<f64>,
    pub focal_length: Option<f64>,
    pub focal_ratio: Option<f64>,
    pub image_circle_diameter: Option<f64>,
    pub spectral_config: Option<Value>,
    pub filter_change_time_seconds: Option<f64>,
    pub spectral_min_wavelength_nm: Option<f64>,
    pub spectral_max_wavelength_nm: Option<f64>,
    pub status: Option<String>,
}
