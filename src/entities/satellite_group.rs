use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteGroup {
    pub id: String,
    pub title: String,
    pub details: Option<String>,
    pub user_id: Option<String>,
    pub user_group_id: Option<String>,
    pub username: Option<String>,
    pub satellite_ids: Option<Vec<String>>,
    pub is_favorited: Option<bool>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateSatelliteGroupRequest {
    pub title: String,
    pub details: Option<String>,
    pub satellite_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSatelliteGroupRequest {
    pub id: String,
    pub title: Option<String>,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteGroupMembersRequest {
    pub satellite_ids: Vec<String>,
}
