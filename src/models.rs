use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SpdxExpr {
    License(String),
    And(Box<SpdxExpr>, Box<SpdxExpr>),
    Or(Box<SpdxExpr>, Box<SpdxExpr>),
    With(Box<SpdxExpr>, String), // License WITH exception
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct License {
    pub id: String,
    pub name: String,
    pub copyleft_strength: CopyleftStrength,
    pub is_osi_approved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CopyleftStrength {
    None,           // Permissive licenses
    Weak,           // LGPL-style
    Strong,         // GPL-style
    Network,        // AGPL-style
    Unknown,        // Custom/unrecognized licenses
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
    pub possible_licenses: Vec<License>,
    pub strongest_copyleft: CopyleftStrength,
    pub recommended_choice: Option<License>,
    pub risk_level: RiskLevel,
    pub compliance_notes: Vec<String>,
    pub conflicts: Vec<String>,
}

impl fmt::Display for CopyleftStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyleftStrength::None => write!(f, "Permissive"),
            CopyleftStrength::Weak => write!(f, "Weak Copyleft"),
            CopyleftStrength::Strong => write!(f, "Strong Copyleft"),
            CopyleftStrength::Network => write!(f, "Network Copyleft"),
            CopyleftStrength::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for LicenseAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== License Analysis ===")?;
        writeln!(f, "Original Expression: {}", self.original_expression)?;
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

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}