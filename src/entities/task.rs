use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Pending,
    Canceled,
    Scheduled,
    Succeeded,
    Failed
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    #[serde(rename = "type")]
    pub task_type: String,
    pub status: TaskStatus,
    #[serde(rename = "creationEpoch")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: DateTime<Utc>,
    pub task_start: DateTime<Utc>,
    pub task_stop: DateTime<Utc>,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub telescope_id: String,
    pub telescope_name: Option<String>,
    pub ground_station_id: String,
    pub ground_station_name: Option<String>,
    pub priority: i32,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_stop: Option<DateTime<Utc>>,
    pub range_km: Option<f64>,
    pub range_rate_km_s: Option<f64>,
    pub right_ascension: Option<f64>,
    pub right_ascension_rate: Option<f64>,
    pub declination: Option<f64>,
    pub declination_rate: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateRequest {
    pub id: String,
    pub status: TaskStatus,
    pub priority: Option<i32>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_stop: Option<DateTime<Utc>>,
}
