use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAccount {
    pub id: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub user_group_id: Option<String>,
    pub role: Option<String>,
    pub tier: Option<String>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "lastLoginEpoch")]
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub default_timezone: Option<String>,
    pub default_units: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub email_notifications: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePreferencesRequest {
    pub default_timezone: Option<String>,
    pub default_units: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub email_notifications: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub role: Option<String>,
    #[serde(rename = "joinedEpoch")]
    pub joined_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddGroupMemberRequest {
    pub email: String,
    pub role: Option<String>,
}
