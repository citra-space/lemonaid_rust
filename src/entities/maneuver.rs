use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Maneuver {
    pub id: String,
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub status: Option<String>,
    pub epoch: Option<DateTime<Utc>>,
    pub magnitude: Option<f64>,
    pub radial_magnitude: Option<f64>,
    pub in_track_magnitude: Option<f64>,
    pub cross_track_magnitude: Option<f64>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateManeuverRequest {
    pub satellite_id: String,
    pub epoch: DateTime<Utc>,
    pub magnitude: Option<f64>,
    pub radial_magnitude: Option<f64>,
    pub in_track_magnitude: Option<f64>,
    pub cross_track_magnitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateManeuverRequest {
    pub status: Option<String>,
    pub epoch: Option<DateTime<Utc>>,
    pub magnitude: Option<f64>,
    pub radial_magnitude: Option<f64>,
    pub in_track_magnitude: Option<f64>,
    pub cross_track_magnitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManeuverListQuery {
    pub satellite_id: Option<String>,
    pub status: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
