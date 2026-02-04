use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Elset {
    pub id: String,
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub user_id: Option<String>,
    pub user_group_id: Option<String>,
    pub user_group_name: Option<String>,
    pub username: Option<String>,
    pub user_tier: Option<String>,
    pub review_status: Option<String>,
    pub epoch: DateTime<Utc>,
    #[serde(rename = "type")]
    pub elset_type: Option<String>,
    pub mean_motion: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
    #[serde(rename = "bStar")]
    pub bstar: Option<f64>,
    pub mean_motion_dot: Option<f64>,
    pub mean_motion_dot_dot: Option<f64>,
    pub ballistic_coefficient: Option<f64>,
    pub srp_coefficient: Option<f64>,
    pub rms: Option<f64>,
    pub tle: Option<Vec<String>>,
    pub semi_major_axis: Option<f64>,
    // Legacy fields for compatibility
    pub norad_id: Option<i64>,
    pub element_set_no: Option<i32>,
    pub rev_at_epoch: Option<i32>,
    pub period_minutes: Option<f64>,
    pub apogee_km: Option<f64>,
    pub perigee_km: Option<f64>,
    pub source: Option<String>,
    pub is_xp: Option<bool>,
    pub rejected: Option<bool>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateElsetRequest {
    pub satellite_id: String,
    pub epoch: DateTime<Utc>,
    pub mean_motion: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
    #[serde(rename = "bStar")]
    pub bstar: Option<f64>,
    pub mean_motion_dot: Option<f64>,
    pub mean_motion_dot_dot: Option<f64>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ElsetCount {
    pub hour: DateTime<Utc>,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoScatterPoint {
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub longitude: f64,
    pub inclination: f64,
    pub eccentricity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeoScatterPoint {
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub semi_major_axis: f64,
    pub inclination: f64,
    pub eccentricity: f64,
    pub apogee_km: Option<f64>,
    pub perigee_km: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ElsetHistoryQuery {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub source: Option<String>,
    pub limit: Option<i64>,
}
