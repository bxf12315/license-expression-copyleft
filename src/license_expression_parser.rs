use std::collections::HashMap;
use crate::models::{OldLicense, CopyleftStrength, SpdxExpr, RiskLevel, LicenseAnalysis};
use crate::license_database;

#[derive(Debug)]
pub struct LicenseExpressionParser {
    license_db: HashMap<String, OldLicense>,
}

impl LicenseExpressionParser {
    pub fn new() -> Self {
        LicenseExpressionParser {
            license_db: license_database::init_license_database(),
        }
    }

    pub fn parse(&self, expression: &str) -> Result<SpdxExpr, String> {
        let tokens = self.tokenize(expression)?;
        self.parse_or_expression(&tokens, &mut 0)
    }

    fn tokenize(&self, expression: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut paren_depth = 0;

        for ch in expression.chars() {
            match ch {
                '(' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                    paren_depth += 1;
                }
                ')' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                    paren_depth -= 1;
                }
                ' ' | '\t' | '\n' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                _ => {
                    current_token.push(ch);
                }
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        if paren_depth != 0 {
            return Err("Mismatched parentheses".to_string());
        }

        Ok(tokens)
    }

    fn parse_or_expression(&self, tokens: &[String], pos: &mut usize) -> Result<SpdxExpr, String> {
        let mut left = self.parse_and_expression(tokens, pos)?;

        while *pos < tokens.len() && tokens[*pos].to_uppercase() == "OR" {
            *pos += 1; // consume OR
            let right = self.parse_and_expression(tokens, pos)?;
            left = SpdxExpr::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_and_expression(&self, tokens: &[String], pos: &mut usize) -> Result<SpdxExpr, String> {
        let mut left = self.parse_with_expression(tokens, pos)?;

        while *pos < tokens.len() && tokens[*pos].to_uppercase() == "AND" {
            *pos += 1; // consume AND
            let right = self.parse_with_expression(tokens, pos)?;
            left = SpdxExpr::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_with_expression(&self, tokens: &[String], pos: &mut usize) -> Result<SpdxExpr, String> {
        let mut left = self.parse_primary(tokens, pos)?;

        while *pos < tokens.len() && tokens[*pos].to_uppercase() == "WITH" {
            *pos += 1; // consume WITH
            if *pos >= tokens.len() {
                return Err("Expected exception after WITH".to_string());
            }
            let exception = tokens[*pos].clone();
            *pos += 1;
            left = SpdxExpr::With(Box::new(left), exception);
        }

        Ok(left)
    }

    fn parse_primary(&self, tokens: &[String], pos: &mut usize) -> Result<SpdxExpr, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        if tokens[*pos] == "(" {
            *pos += 1; // consume (
            let expr = self.parse_or_expression(tokens, pos)?;
            if *pos >= tokens.len() || tokens[*pos] != ")" {
                return Err("Expected closing parenthesis".to_string());
            }
            *pos += 1; // consume )
            Ok(expr)
        } else {
            let license_id = tokens[*pos].clone();
            *pos += 1;
            Ok(SpdxExpr::License(license_id))
        }
    }

    pub fn analyze(&self, expression: &str) -> LicenseAnalysis {
        print!("expressionexpressionexpressionexpressionexpression--------- {:?}", expression);
        let parsed = match self.parse(expression) {
            Ok(expr) => Some(expr),
            Err(er) => {print!( "ttttttttttttt {}", er);None},
        };

        print!("123123123123123123123--------- {:?}", parsed.clone());

        let possible_licenses = if let Some(ref expr) = parsed {
            self.evaluate_expression(expr)
        } else {
            Vec::new()
        };

        let strongest_copyleft = self.find_strongest_copyleft(&possible_licenses);
        let recommended_choice = self.choose_recommended_license(&possible_licenses);
        let risk_level = self.assess_risk_level(&strongest_copyleft, &possible_licenses);
        let compliance_notes = self.generate_compliance_notes(&possible_licenses, &recommended_choice);
        let conflicts = self.find_conflicts(&possible_licenses);

        LicenseAnalysis {
            original_expression: expression.to_string(),
            parsed_expression: parsed,
            possible_licenses,
            strongest_copyleft,
            recommended_choice,
            risk_level,
            compliance_notes,
            conflicts,
        }
    }

    fn evaluate_expression(&self, expr: &SpdxExpr) -> Vec<OldLicense> {
        match expr {
            SpdxExpr::License(id) => {
                if let Some(license) = self.license_db.get(id) {
                    vec![license.clone()]
                } else {
                    // Handle unknown licenses
                    vec![OldLicense {
                        id: id.clone(),
                        name: format!("Unknown License: {}", id),
                        copyleft_strength: CopyleftStrength::Unknown,
                        is_osi_approved: false,
                    }]
                }
            }
            SpdxExpr::Or(left, right) => {
                let mut result = self.evaluate_expression(left);
                result.extend(self.evaluate_expression(right));
                result
            }
            SpdxExpr::And(left, right) => {
                let left_licenses = self.evaluate_expression(left);
                let right_licenses = self.evaluate_expression(right);
                self.find_compatible_licenses(&left_licenses, &right_licenses)
            }
            SpdxExpr::With(license_expr, _exception) => {
                // For now, treat WITH expressions as the base license
                // In a full implementation, you'd handle specific exceptions
                self.evaluate_expression(license_expr)
            }
        }
    }

    fn find_compatible_licenses(&self, left: &[OldLicense], right: &[OldLicense]) -> Vec<OldLicense> {
        let mut compatible = Vec::new();

        for left_lic in left {
            for right_lic in right {
                if self.are_licenses_compatible(left_lic, right_lic) {
                    let stronger = self.choose_stronger_license(left_lic, right_lic);
                    if !compatible.iter().any(|l: &OldLicense| l.id == stronger.id) {
                        compatible.push(stronger);
                    }
                }
            }
        }

        // If no compatible licenses found, return the stronger of all combinations
        if compatible.is_empty() {
            for left_lic in left {
                for right_lic in right {
                    let stronger = self.choose_stronger_license(left_lic, right_lic);
                    if !compatible.iter().any(|l: &OldLicense| l.id == stronger.id) {
                        compatible.push(stronger);
                    }
                }
            }
        }

        compatible
    }

    fn are_licenses_compatible(&self, a: &OldLicense, b: &OldLicense) -> bool {
        // Basic compatibility rules
        match (&a.copyleft_strength, &b.copyleft_strength) {
            // Same license is always compatible
            _ if a.id == b.id => true,

            // Permissive licenses are compatible with everything
            (CopyleftStrength::None, _) | (_, CopyleftStrength::None) => true,

            // Weak copyleft can be combined with strong copyleft (results in strong)
            (CopyleftStrength::Weak, CopyleftStrength::Strong) |
            (CopyleftStrength::Strong, CopyleftStrength::Weak) => true,

            // Network copyleft can combine with other copylefts
            (CopyleftStrength::Network, CopyleftStrength::Strong) |
            (CopyleftStrength::Strong, CopyleftStrength::Network) => true,

            // Specific GPL compatibility rules
            _ => self.check_gpl_compatibility(a, b),
        }
    }

    fn check_gpl_compatibility(&self, a: &OldLicense, b: &OldLicense) -> bool {
        // Handle specific GPL version compatibility
        match (a.id.as_str(), b.id.as_str()) {
            // GPL v2 only vs GPL v3+ incompatibility
            ("GPL-2.0-only", id) if id.contains("GPL-3.0") => false,
            (id, "GPL-2.0-only") if id.contains("GPL-3.0") => false,

            // GPL v2+ can upgrade to v3+
            ("GPL-2.0-or-later", id) if id.contains("GPL-3.0") => true,
            (id, "GPL-2.0-or-later") if id.contains("GPL-3.0") => true,

            // LGPL v3+ is compatible with GPL v3+
            (id1, id2) if id1.contains("LGPL-3.0") && id2.contains("GPL-3.0") => true,
            (id1, id2) if id1.contains("GPL-3.0") && id2.contains("LGPL-3.0") => true,

            // Default to incompatible for strong copyleft combinations
            _ if matches!(a.copyleft_strength, CopyleftStrength::Strong) &&
                matches!(b.copyleft_strength, CopyleftStrength::Strong) => false,

            _ => true,
        }
    }

    fn choose_stronger_license(&self, a: &OldLicense, b: &OldLicense) -> OldLicense {
        let a_strength = self.copyleft_strength_order(&a.copyleft_strength);
        let b_strength = self.copyleft_strength_order(&b.copyleft_strength);

        if a_strength >= b_strength {
            a.clone()
        } else {
            b.clone()
        }
    }

    fn copyleft_strength_order(&self, strength: &CopyleftStrength) -> u8 {
        match strength {
            CopyleftStrength::None => 0,
            CopyleftStrength::Weak => 1,
            CopyleftStrength::Strong => 2,
            CopyleftStrength::Network => 3,
            CopyleftStrength::Unknown => 4,
        }
    }

    fn find_strongest_copyleft(&self, licenses: &[OldLicense]) -> CopyleftStrength {
        licenses.iter()
            .map(|l| &l.copyleft_strength)
            .max_by_key(|s| self.copyleft_strength_order(s))
            .unwrap_or(&CopyleftStrength::None)
            .clone()
    }

    fn choose_recommended_license(&self, licenses: &[OldLicense]) -> Option<OldLicense> {
        if licenses.is_empty() {
            return None;
        }

        // Prefer permissive licenses, then weak copyleft, then strong copyleft
        let mut sorted_licenses = licenses.to_vec();
        sorted_licenses.sort_by_key(|l| self.copyleft_strength_order(&l.copyleft_strength));

        sorted_licenses.into_iter().next()
    }

    fn assess_risk_level(&self, strongest: &CopyleftStrength, licenses: &[OldLicense]) -> RiskLevel {
        if licenses.is_empty() {
            return RiskLevel::Critical;
        }

        match strongest {
            CopyleftStrength::None => RiskLevel::Low,
            CopyleftStrength::Weak => RiskLevel::Medium,
            CopyleftStrength::Strong => RiskLevel::High,
            CopyleftStrength::Network => RiskLevel::Critical,
            CopyleftStrength::Unknown => RiskLevel::Unknown,
        }
    }

    fn generate_compliance_notes(&self, licenses: &[OldLicense], recommended: &Option<OldLicense>) -> Vec<String> {
        let mut notes = Vec::new();

        if licenses.is_empty() {
            notes.push("No compatible licenses found - this is a licensing conflict!".to_string());
            return notes;
        }

        if let Some(rec) = recommended {
            notes.push(format!("Recommended license choice: {}", rec.id));

            match rec.copyleft_strength {
                CopyleftStrength::Strong => {
                    notes.push("Strong copyleft: All derivative works must use compatible licenses".to_string());
                    notes.push("Required: Provide complete source code upon distribution".to_string());
                    notes.push("Caution: Static linking may affect entire codebase".to_string());
                }
                CopyleftStrength::Weak => {
                    notes.push("Weak copyleft: Only modifications to this component must be open-sourced".to_string());
                    notes.push("Dynamic linking generally acceptable".to_string());
                }
                CopyleftStrength::None => {
                    notes.push("✅ Permissive license: Minimal compliance requirements".to_string());
                    notes.push("Required: Include license notice and attribution".to_string());
                }
                CopyleftStrength::Network => {
                    notes.push("Network copyleft: Source must be provided even for network services".to_string());
                    notes.push("Applies to SaaS and web services".to_string());
                }
                CopyleftStrength::Unknown => {
                    notes.push("Unknown license: Manual legal review required".to_string());
                }
            }
        }

        if licenses.len() > 1 {
            let alternatives: Vec<String> = licenses.iter()
                .filter(|l| Some(*l) != recommended.as_ref())
                .map(|l| l.id.clone())
                .collect();

            if !alternatives.is_empty() {
                notes.push(format!("🔄 Alternative licenses available: {}", alternatives.join(", ")));
            }
        }

        notes
    }

    fn find_conflicts(&self, licenses: &[OldLicense]) -> Vec<String> {
        let mut conflicts = Vec::new();

        if licenses.is_empty() {
            conflicts.push("Complete licensing conflict - no compatible licenses found".to_string());
        }

        // Check for specific known conflicts
        let has_gpl2_only = licenses.iter().any(|l| l.id == "GPL-2.0-only");
        let has_gpl3 = licenses.iter().any(|l| l.id.contains("GPL-3.0"));

        if has_gpl2_only && has_gpl3 {
            conflicts.push("GPL-2.0-only is incompatible with GPL-3.0+ licenses".to_string());
        }

        conflicts
    }
}