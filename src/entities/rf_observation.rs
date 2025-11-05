use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RFDetection {
    pub center_frequency_hz: i64,
    pub bandwidth_hz: i64,
    pub strength_dbm: f64,
    pub snr_db: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RFPowerSpectralDensity {
    pub frequency_hz: Vec<i64>,
    pub power_dbm_per_hz: Vec<f64>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RFCaptureData {
    pub detections: Vec<RFDetection>,
    pub power_spectral_density: RFPowerSpectralDensity
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRFCaptureRequest {
    pub antenna_id: String,
    pub capture_start: DateTime<Utc>,
    pub capture_end: DateTime<Utc>,
    pub data: RFCaptureData,
    pub task_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RFCapture {
    pub id: String,
    pub antenna_id: String,
    pub user_id: String,
    pub capture_start: DateTime<Utc>,
    pub capture_end: DateTime<Utc>,
    pub data: RFCaptureData,
    pub detection_count: usize,
    pub task_id: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RFCaptureSummary {
    pub id: String,
    pub antenna_id: String,
    pub user_id: String,
    pub capture_start: DateTime<Utc>,
    pub capture_end: DateTime<Utc>,
    pub detection_count: usize,
    pub task_id: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
}