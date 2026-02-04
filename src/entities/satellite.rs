use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Satellite {
    pub id: String,
    pub name: String,
    #[serde(rename = "noradCatId")]
    pub norad_id: Option<i64>,
    #[serde(rename = "creationEpoch")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "decayEpoch")]
    pub decay_date: Option<DateTime<Utc>>,
    #[serde(rename = "launchDateEpoch")]
    pub launch_date: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub object_type: Option<String>,
    pub origin_type: Option<String>,
    pub maneuver_capability: Option<String>,
    pub optical_cross_section: Option<f64>,
    pub radar_cross_section: Option<f64>,
    pub alias_count: Option<i64>,
    pub elset_count: Option<i64>,
    pub transmission_count: Option<i64>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub country_iso: Option<String>,
    pub site: Option<String>,
    pub launch_site_name: Option<String>,
    pub orbit_regime: Option<String>,
    pub orbit_regime_full_name: Option<String>,
    pub altitude_km: Option<f64>,
    pub altitude_display: Option<String>,
    pub coverage_type: Option<String>,
    pub coverage_description: Option<String>,
    // Legacy fields for backwards compatibility
    pub international_designator: Option<String>,
    pub rcs_size: Option<String>,
    pub period_minutes: Option<f64>,
    pub inclination_deg: Option<f64>,
    pub apogee_km: Option<f64>,
    pub perigee_km: Option<f64>,
    pub semi_major_axis_km: Option<f64>,
    pub eccentricity: Option<f64>,
    #[serde(rename = "updateEpoch")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteOverview {
    #[serde(default)]
    pub active_satellite_count: i64,
    #[serde(default)]
    pub decayed_satellite_count: i64,
    #[serde(default)]
    pub antenna_count: i64,
    #[serde(default)]
    pub telescope_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteListQuery {
    pub ids: Option<Vec<String>>,
    pub search: Option<String>,
    pub country: Option<String>,
    pub object_type: Option<String>,
    pub include_decayed: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SatellitePageResponse {
    #[serde(default)]
    pub items: Vec<Satellite>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SatellitePaginatedResponse {
    #[serde(default)]
    pub satellites: Vec<Satellite>,
    #[serde(default)]
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CountryCount {
    pub country: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    #[serde(default)]
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CloseApproach {
    pub satellite_id: String,
    pub satellite_name: Option<String>,
    pub other_satellite_id: String,
    pub other_satellite_name: Option<String>,
    pub epoch: DateTime<Utc>,
    pub distance_km: f64,
    pub relative_velocity_km_s: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelativeState {
    pub epoch: DateTime<Utc>,
    pub range_km: f64,
    pub range_rate_km_s: f64,
    pub in_track_km: Option<f64>,
    pub cross_track_km: Option<f64>,
    pub radial_km: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroundTrackPoint {
    pub epoch: DateTime<Utc>,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_km: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationBounds {
    pub satellite_id: String,
    pub first_observation: Option<DateTime<Utc>>,
    pub last_observation: Option<DateTime<Utc>>,
    pub observation_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResidualsRequest {
    pub observations: Vec<ObservationResidual>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObservationResidual {
    pub epoch: DateTime<Utc>,
    pub right_ascension_deg: f64,
    pub declination_deg: f64,
    pub sensor_latitude_deg: f64,
    pub sensor_longitude_deg: f64,
    pub sensor_altitude_km: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResidualResult {
    pub epoch: DateTime<Utc>,
    pub right_ascension_residual_arcsec: f64,
    pub declination_residual_arcsec: f64,
    pub total_residual_arcsec: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrbitalElements {
    pub epoch: DateTime<Utc>,
    pub semi_major_axis_km: f64,
    pub eccentricity: f64,
    pub inclination_deg: f64,
    pub raan_deg: f64,
    pub argument_of_perigee_deg: f64,
    pub mean_anomaly_deg: f64,
    pub period_minutes: Option<f64>,
    pub apogee_km: Option<f64>,
    pub perigee_km: Option<f64>,
}
