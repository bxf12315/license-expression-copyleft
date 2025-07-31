use std::collections::HashMap;
use crate::models::{OldLicense, CopyleftStrength, NewCopyleftStrength};
use crate::license::License;
use serde_json;
use std::fmt;

pub fn init_license_database() -> HashMap<String, OldLicense> {
    let mut license_db = HashMap::new();
    // Common SPDX licenses with their characteristics
    let licenses = vec![
        // Strong Copyleft
        ("GPL-2.0", "GNU General Public License v2.0", CopyleftStrength::Strong, true),
        ("GPL-2.0+", "GNU General Public License v2.0 or later", CopyleftStrength::Strong, true),
        ("GPL-2.0-only", "GNU General Public License v2.0 only", CopyleftStrength::Strong, true),
        ("GPL-2.0-or-later", "GNU General Public License v2.0 or later", CopyleftStrength::Strong, true),
        ("GPL-3.0", "GNU General Public License v3.0", CopyleftStrength::Strong, true),
        ("GPL-3.0+", "GNU General Public License v3.0 or later", CopyleftStrength::Strong, true),
        ("GPL-3.0-only", "GNU General Public License v3.0 only", CopyleftStrength::Strong, true),
        ("GPL-3.0-or-later", "GNU General Public License v3.0 or later", CopyleftStrength::Strong, true),

        // Network Copyleft
        ("AGPL-3.0", "GNU Affero General Public License v3.0", CopyleftStrength::Network, true),
        ("AGPL-3.0+", "GNU Affero General Public License v3.0 or later", CopyleftStrength::Network, true),
        ("AGPL-3.0-only", "GNU Affero General Public License v3.0 only", CopyleftStrength::Network, true),
        ("AGPL-3.0-or-later", "GNU Affero General Public License v3.0 or later", CopyleftStrength::Network, true),

        // Weak Copyleft
        ("LGPL-2.1", "GNU Lesser General Public License v2.1", CopyleftStrength::Weak, true),
        ("LGPL-2.1+", "GNU Lesser General Public License v2.1 or later", CopyleftStrength::Weak, true),
        ("LGPL-2.1-only", "GNU Lesser General Public License v2.1 only", CopyleftStrength::Weak, true),
        ("LGPL-2.1-or-later", "GNU Lesser General Public License v2.1 or later", CopyleftStrength::Weak, true),
        ("LGPL-3.0", "GNU Lesser General Public License v3.0", CopyleftStrength::Weak, true),
        ("LGPL-3.0+", "GNU Lesser General Public License v3.0 or later", CopyleftStrength::Weak, true),
        ("LGPL-3.0-only", "GNU Lesser General Public License v3.0 only", CopyleftStrength::Weak, true),
        ("LGPL-3.0-or-later", "GNU Lesser General Public License v3.0 or later", CopyleftStrength::Weak, true),
        ("MPL-2.0", "Mozilla Public License 2.0", CopyleftStrength::Weak, true),
        ("EPL-2.0", "Eclipse Public License 2.0", CopyleftStrength::Weak, true),
        ("CDDL-1.0", "Common Development and Distribution License 1.0", CopyleftStrength::Weak, true),

        // Permissive
        ("MIT", "MIT License", CopyleftStrength::None, true),
        ("Apache-2.0", "Apache License 2.0", CopyleftStrength::None, true),
        ("BSD-2-Clause", "BSD 2-Clause License", CopyleftStrength::None, true),
        ("BSD-3-Clause", "BSD 3-Clause License", CopyleftStrength::None, true),
        ("ISC", "ISC License", CopyleftStrength::None, true),
        ("Unlicense", "The Unlicense", CopyleftStrength::None, true),
        ("0BSD", "BSD Zero Clause License", CopyleftStrength::None, true),

        // Other notable licenses
        ("CC0-1.0", "Creative Commons Zero v1.0 Universal", CopyleftStrength::None, true),
        ("WTFPL", "Do What The F*ck You Want To Public License", CopyleftStrength::None, false),
    ];

    for (id, name, strength, osi) in licenses {
        license_db.insert(id.to_string(), OldLicense {
            id: id.to_string(),
            name: name.to_string(),
            copyleft_strength: strength,
            is_osi_approved: osi,
        });
    }
    license_db
}

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
    pub is_osi_approved: bool,
}

/// Loads all licenses from index.json file and returns them as a HashMap
/// This function reads the JSON file and maps the data to NewLicense format using NewCopyleftStrength
pub fn load_licenses_from_json() -> Result<HashMap<String, NewLicense>, LicenseDatabaseError> {
    use std::fs;
    
    // Read the JSON file
    let json_content = fs::read_to_string("index.json")
        .map_err(|e| LicenseDatabaseError::FileReadError(e.to_string()))?;
    
    // Parse the JSON into License structs
    let licenses: Vec<License> = serde_json::from_str(&json_content)
        .map_err(|e| LicenseDatabaseError::JsonParseError(e.to_string()))?;
    
    let mut license_db = HashMap::new();
    
    for license in licenses {
        // Map category to NewCopyleftStrength using exact category mapping
        let copyleft_strength = match license.category.as_str() {
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
        
        // Determine OSI approval status based on SPDX license key
        let is_osi_approved = match license.spdx_license_key.as_deref() {
            Some("GPL-2.0") | Some("GPL-2.0-only") | Some("GPL-2.0-or-later") => true,
            Some("GPL-3.0") | Some("GPL-3.0-only") | Some("GPL-3.0-or-later") => true,
            Some("LGPL-2.1") | Some("LGPL-2.1-only") | Some("LGPL-2.1-or-later") => true,
            Some("LGPL-3.0") | Some("LGPL-3.0-only") | Some("LGPL-3.0-or-later") => true,
            Some("AGPL-3.0") | Some("AGPL-3.0-only") | Some("AGPL-3.0-or-later") => true,
            Some("MIT") => true,
            Some("Apache-2.0") => true,
            Some("BSD-2-Clause") => true,
            Some("BSD-3-Clause") => true,
            Some("ISC") => true,
            Some("MPL-2.0") => true,
            Some("EPL-2.0") => true,
            Some("CDDL-1.0") => true,
            Some("CC0-1.0") => true,
            Some("Unlicense") => true,
            Some("0BSD") => true,
            _ => false,
        };

        // Create NewLicense from License using NewCopyleftStrength
        let new_license = NewLicense {
            id: license.license_key.clone(),
            name: license.spdx_license_key
                .as_ref()
                .unwrap_or(&license.license_key)
                .to_string(),
            copyleft_strength,
            is_osi_approved,
        };
        
        license_db.insert(license.license_key, new_license);
    }
    
    Ok(license_db)
}