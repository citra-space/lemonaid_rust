use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ODObservation {
    pub epoch: DateTime<Utc>,
    pub right_ascension_deg: f64,
    pub declination_deg: f64,
    pub sensor_latitude_deg: f64,
    pub sensor_longitude_deg: f64,
    pub sensor_altitude_km: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ODRequest {
    pub satellite_id: Option<String>,
    pub observations: Vec<ODObservation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ODResult {
    pub epoch: DateTime<Utc>,
    pub semi_major_axis_km: f64,
    pub eccentricity: f64,
    pub inclination_deg: f64,
    pub raan_deg: f64,
    pub argument_of_perigee_deg: f64,
    pub mean_anomaly_deg: f64,
    pub position_eci_km: [f64; 3],
    pub velocity_eci_km_s: [f64; 3],
    pub rms_residual_arcsec: Option<f64>,
    pub covariance: Option<Vec<Vec<f64>>>,
}
