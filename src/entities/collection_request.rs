use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CollectionRequestType {
    Track,
    Tdoa,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRequest {
    pub id: String,
    #[serde(rename = "type")]
    pub request_type: CollectionRequestType,
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub user_id: String,
    pub status: String,
    pub priority: Option<i32>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub notes: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionRequestRequest {
    #[serde(rename = "type")]
    pub request_type: CollectionRequestType,
    pub satellite_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub priority: Option<i32>,
    pub notes: Option<String>,
}
