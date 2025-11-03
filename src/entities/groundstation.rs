use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Groundstation {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub user_id: String,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "lastConnectionEpoch")]
    pub last_connected_at: Option<DateTime<Utc>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GroundstationListResponse {
    pub ground_stations: Vec<Groundstation>
}