use std::fmt;
use crate::license_database::NewLicense;

#[derive(Debug, Clone, PartialEq)]
pub enum SpdxExpr {
    License(String),
    And(Box<SpdxExpr>, Box<SpdxExpr>),
    Or(Box<SpdxExpr>, Box<SpdxExpr>),
    With(Box<SpdxExpr>, String), // License WITH exception
}

/// New copyleft strength categories based on detailed license classifications
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NewCopyleftStrength {
    /// Contributor License Agreement (CLA)
    /// Describes contribution acceptance rules for software projects
    CLA,
    
    /// Commercial
    /// Third-party proprietary software offered under a direct commercial license
    Commercial,
    
    /// Copyleft
    /// Open source software with copyleft license requiring same license terms for redistributions
    Copyleft,
    
    /// Copyleft Limited
    /// Requires redistribution of source code with limited obligations according to license-specific rules
    CopyleftLimited,
    
    /// Free Restricted
    /// Permissive-style license with restrictions on usage or redistribution
    FreeRestricted,
    
    /// Patent License
    /// License that applies to patents rather than specific software
    PatentLicense,
    
    /// Permissive
    /// Open Source software under non-copyleft licenses requiring attribution
    Permissive,
    
    /// Proprietary Free
    /// Proprietary Free software with specific terms and conditions
    ProprietaryFree,
    
    /// Public Domain
    /// Open source software made available without explicit obligations
    PublicDomain,
    
    /// Source-available
    /// Software released through source code distribution model without necessarily meeting open-source criteria
    SourceAvailable,
    
    /// Unstated License
    /// Third-party software with copyright notice but no stated license
    UnstatedLicense,
}

#[derive(Debug, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

#[derive(Debug)]
pub struct LicenseAnalysis {
    pub original_expression: String,
    pub parsed_expression: Option<SpdxExpr>,
    pub possible_licenses: Vec<NewLicense>,
    pub strongest_copyleft: NewCopyleftStrength,
    pub recommended_choice: Option<NewLicense>,
    pub risk_level: RiskLevel,
    pub compliance_notes: Vec<String>,
    pub conflicts: Vec<String>,
}

impl fmt::Display for LicenseAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "License Analysis for: {}", self.original_expression)?;
        writeln!(f, "Risk Level: {}", self.risk_level)?;
        writeln!(f, "Strongest Copyleft: {}", self.strongest_copyleft)?;

        if let Some(ref expr) = self.parsed_expression {
            writeln!(f, "Parsed Expression: {:?}", expr)?;
        }

        if !self.possible_licenses.is_empty() {
            writeln!(f, "Possible Licenses ({}):", self.possible_licenses.len())?;
            for license in &self.possible_licenses {
                writeln!(f, "  - {} ({})", license.id, license.copyleft_strength)?;
            }
        } else {
            writeln!(f, "Possible Licenses: None (CONFLICT)")?;
        }

        if let Some(ref recommended) = self.recommended_choice {
            writeln!(f, "Recommended Choice: {}", recommended.id)?;
        }

        if !self.compliance_notes.is_empty() {
            writeln!(f, "Compliance Notes:")?;
            for note in &self.compliance_notes {
                writeln!(f, "  {}", note)?;
            }
        }

        if !self.conflicts.is_empty() {
            writeln!(f, "CONFLICTS DETECTED:")?;
            for conflict in &self.conflicts {
                writeln!(f, "  {}", conflict)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
            RiskLevel::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for NewCopyleftStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NewCopyleftStrength::CLA => write!(f, "Contributor License Agreement"),
            NewCopyleftStrength::Commercial => write!(f, "Commercial"),
            NewCopyleftStrength::Copyleft => write!(f, "Copyleft"),
            NewCopyleftStrength::CopyleftLimited => write!(f, "Copyleft Limited"),
            NewCopyleftStrength::FreeRestricted => write!(f, "Free Restricted"),
            NewCopyleftStrength::PatentLicense => write!(f, "Patent License"),
            NewCopyleftStrength::Permissive => write!(f, "Permissive"),
            NewCopyleftStrength::ProprietaryFree => write!(f, "Proprietary Free"),
            NewCopyleftStrength::PublicDomain => write!(f, "Public Domain"),
            NewCopyleftStrength::SourceAvailable => write!(f, "Source-available"),
            NewCopyleftStrength::UnstatedLicense => write!(f, "Unstated License"),
        }
    }
}

/// Returns a numeric value representing the strength order of NewCopyleftStrength variants
/// Higher values indicate stronger copyleft requirements
/// Ordered by risk level from highest (avoid) to lowest (safe)
pub fn new_copyleft_strength_order(strength: &NewCopyleftStrength) -> u8 {
    match strength {
        // Avoid using - highest risk
        NewCopyleftStrength::UnstatedLicense => 10,
        NewCopyleftStrength::Commercial => 9,
        
        // Mandatory open source - high risk
        NewCopyleftStrength::Copyleft => 8,
        NewCopyleftStrength::SourceAvailable => 7,
        
        // Partial restrictions - medium risk
        NewCopyleftStrength::CopyleftLimited => 6,
        NewCopyleftStrength::FreeRestricted => 5,
        NewCopyleftStrength::ProprietaryFree => 4,
        
        // Special cases
        NewCopyleftStrength::PatentLicense => 3,
        NewCopyleftStrength::CLA => 2,
        
        // Minimal restrictions - low risk
        NewCopyleftStrength::Permissive => 1,
        NewCopyleftStrength::PublicDomain => 0,
    }
}

/// Compares two NewCopyleftStrength values and returns the stronger one
pub fn choose_stronger_new_copyleft(a: &NewCopyleftStrength, b: &NewCopyleftStrength) -> NewCopyleftStrength {
    let a_strength = new_copyleft_strength_order(a);
    let b_strength = new_copyleft_strength_order(b);

    if a_strength >= b_strength {
        a.clone()
    } else {
        b.clone()
    }
}