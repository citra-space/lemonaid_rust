use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub name: String,
    pub category: Option<String>,
    pub center_wavelength_nm: Option<f64>,
    pub bandwidth_nm: Option<f64>,
    pub min_wavelength_nm: Option<f64>,
    pub max_wavelength_nm: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterExpandRequest {
    pub filter_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterExpandResponse {
    pub filters: Vec<Filter>,
}
