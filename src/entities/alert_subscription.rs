use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AlertType {
    Maneuver,
    CloseApproach,
    Decay,
    Launch,
    Observation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Satellite,
    SatelliteGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlertSubscription {
    pub id: String,
    pub user_id: Option<String>,
    pub alert_type: Option<String>,
    pub target_type: Option<String>,
    pub satellite_id: Option<String>,
    pub satellite_group_id: Option<String>,
    pub enabled: Option<bool>,
    pub email_enabled: Option<bool>,
    pub webhook_url: Option<String>,
    pub threshold_value: Option<f64>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlertSubscriptionRequest {
    pub alert_type: AlertType,
    pub target_type: TargetType,
    pub satellite_id: Option<String>,
    pub satellite_group_id: Option<String>,
    pub enabled: Option<bool>,
    pub email_enabled: Option<bool>,
    pub webhook_url: Option<String>,
    pub threshold_value: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAlertSubscriptionRequest {
    pub enabled: Option<bool>,
    pub email_enabled: Option<bool>,
    pub webhook_url: Option<String>,
    pub threshold_value: Option<f64>,
}
