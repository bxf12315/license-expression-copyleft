use std::collections::HashMap;
use crate::models::NewCopyleftStrength;
use crate::license::License;
use serde_json;
use std::fmt;

/// Custom error type for license database operations
#[derive(Debug)]
pub enum LicenseDatabaseError {
    FileReadError(String),
    JsonParseError(String),
}

impl fmt::Display for LicenseDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LicenseDatabaseError::FileReadError(msg) => write!(f, "Failed to read license file: {}", msg),
            LicenseDatabaseError::JsonParseError(msg) => write!(f, "Failed to parse JSON: {}", msg),
        }
    }
}

impl std::error::Error for LicenseDatabaseError {}

/// New license structure using NewCopyleftStrength
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewLicense {
    pub id: String,
    pub name: String,
    pub copyleft_strength: NewCopyleftStrength,
}

/// Loads all licenses from index.json file and returns them as a HashMap
/// This function reads the JSON file and maps the data to NewLicense format using NewCopyleftStrength
pub fn load_licenses_from_json() -> Result<HashMap<String, NewLicense>, LicenseDatabaseError> {
    use std::fs;
    
    // Read the JSON file from parent directory
    let json_content = fs::read_to_string("../index.json")
        .map_err(|e| LicenseDatabaseError::FileReadError(e.to_string()))?;
    
    // Parse the JSON into License structs
    let licenses: Vec<License> = serde_json::from_str(&json_content)
        .map_err(|e| LicenseDatabaseError::JsonParseError(e.to_string()))?;
    
    let mut license_db = HashMap::new();
    
    for license in licenses {
        // Map category to NewCopyleftStrength using exact category mapping
        let copyleft_strength: NewCopyleftStrength = match license.category.as_str() {
            "Copyleft" => NewCopyleftStrength::Copyleft,
            "Copyleft Limited" => NewCopyleftStrength::CopyleftLimited,
            "Permissive" => NewCopyleftStrength::Permissive,
            "Commercial" => NewCopyleftStrength::Commercial,
            "Proprietary Free" => NewCopyleftStrength::ProprietaryFree,
            "Public Domain" => NewCopyleftStrength::PublicDomain,
            "Free Restricted" => NewCopyleftStrength::FreeRestricted,
            "Source-available" => NewCopyleftStrength::SourceAvailable,
            "Unstated License" => NewCopyleftStrength::UnstatedLicense,
            "Patent License" => NewCopyleftStrength::PatentLicense,
            _ => NewCopyleftStrength::UnstatedLicense,
        };
        


        // Create NewLicense from License using NewCopyleftStrength
        let new_license = NewLicense {
            id: license.license_key.clone(),
            name: license.spdx_license_key
                .as_ref()
                .unwrap_or(&license.license_key)
                .to_string(),
            copyleft_strength,
        };
        
        license_db.insert(license.license_key, new_license);
    }
    
    Ok(license_db)
}