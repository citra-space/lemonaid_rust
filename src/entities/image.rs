use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageUploadRequest {
    pub filename: String,
    pub telescope_id: String,
    pub filesize: i64,
    pub task_id: Option<String>,
    pub field_of_view_deg: Option<f64>,
    pub source_limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageUploadResponse {
    pub upload_id: String,
    pub presigned_url: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageStatus {
    pub upload_id: String,
    pub filename: String,
    pub telescope_id: String,
    pub telescope_name: Option<String>,
    pub task_id: Option<String>,
    pub user_id: String,
    pub status: String,
    pub processing_stage: Option<String>,
    pub error_message: Option<String>,
    pub source_count: Option<i32>,
    pub satellite_count: Option<i32>,
    pub filesize: i64,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "processedEpoch")]
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageDataRequest {
    pub binning: Option<i32>,
    pub contrast: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    pub upload_id: String,
    pub width: i32,
    pub height: i32,
    pub data: Vec<Vec<f64>>,
    pub min_value: f64,
    pub max_value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<String>,
}
