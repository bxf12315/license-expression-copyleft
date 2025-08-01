use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct License {
    pub license_key: String,
    pub category: String,
    #[serde(rename = "spdx_license_key")]
    pub spdx_license_key: Option<String>,
    #[serde(rename = "other_spdx_license_keys")]
    pub other_spdx_license_keys: Vec<String>,
    #[serde(rename = "is_exception")]
    pub is_exception: bool,
    #[serde(rename = "is_deprecated")]
    pub is_deprecated: bool,
    pub json: String,
    pub yaml: String,
    pub html: String,
    pub license: String,
}