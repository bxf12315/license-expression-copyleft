use std::collections::HashMap;
use crate::models::{NewCopyleftStrength, SpdxExpr, RiskLevel, LicenseAnalysis};
use crate::license_database::{self, NewLicense};

#[derive(Debug)]
pub struct LicenseExpressionParser {
    license_db: HashMap<String, NewLicense>,
}

impl LicenseExpressionParser {
    pub fn new() -> Self {
        LicenseExpressionParser {
            license_db: license_database::load_licenses_from_json().unwrap_or_default(),
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

    fn evaluate_expression(&self, expr: &SpdxExpr) -> Vec<NewLicense> {
        match expr {
            SpdxExpr::License(id) => {
                if let Some(license) = self.license_db.get(id) {
                    vec![license.clone()]
                } else {
                    // Handle unknown licenses
                    vec![NewLicense {
                        id: id.clone(),
                        name: format!("Unknown License: {}", id),
                        copyleft_strength: NewCopyleftStrength::UnstatedLicense,
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

    fn find_compatible_licenses(&self, left: &[NewLicense], right: &[NewLicense]) -> Vec<NewLicense> {
        let mut compatible = Vec::new();

        for left_lic in left {
            for right_lic in right {
                if self.are_licenses_compatible(left_lic, right_lic) {
                    let stronger = self.choose_stronger_license(left_lic, right_lic);
                    if !compatible.iter().any(|l: &NewLicense| l.id == stronger.id) {
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
                    if !compatible.iter().any(|l: &NewLicense| l.id == stronger.id) {
                        compatible.push(stronger);
                    }
                }
            }
        }

        compatible
    }

    fn are_licenses_compatible(&self, a: &NewLicense, b: &NewLicense) -> bool {
        // Basic compatibility rules based on NewCopyleftStrength risk levels
        match (&a.copyleft_strength, &b.copyleft_strength) {
            // Same license is always compatible
            _ if a.id == b.id => true,

            // 🟢 低风险 - 完全兼容
            (NewCopyleftStrength::PublicDomain, _) | (_, NewCopyleftStrength::PublicDomain) => true,
            (NewCopyleftStrength::Permissive, _) | (_, NewCopyleftStrength::Permissive) => true,

            // ⚖️ 特殊情况 - 通常兼容
            (NewCopyleftStrength::CLA, NewCopyleftStrength::CLA) => true,
            (NewCopyleftStrength::CLA, _) | (_, NewCopyleftStrength::CLA) => true,
            (NewCopyleftStrength::PatentLicense, _) | (_, NewCopyleftStrength::PatentLicense) => true,

            // 🟡 中等风险 - 有限兼容
            (NewCopyleftStrength::ProprietaryFree, NewCopyleftStrength::ProprietaryFree) => true,
            (NewCopyleftStrength::FreeRestricted, NewCopyleftStrength::FreeRestricted) => true,

            // 🟡 CopyleftLimited组合规则 - 需要具体检查
            (NewCopyleftStrength::CopyleftLimited, NewCopyleftStrength::Copyleft) |
            (NewCopyleftStrength::Copyleft, NewCopyleftStrength::CopyleftLimited) => {
                // LGPL与GPL的兼容性需要具体版本判断
                self.check_specific_compatibility(a, b)
            },
            (NewCopyleftStrength::CopyleftLimited, NewCopyleftStrength::Permissive) |
            (NewCopyleftStrength::Permissive, NewCopyleftStrength::CopyleftLimited) => true,
            (NewCopyleftStrength::CopyleftLimited, NewCopyleftStrength::PublicDomain) |
            (NewCopyleftStrength::PublicDomain, NewCopyleftStrength::CopyleftLimited) => true,

            // 🔴 高风险 - 严格限制
            (NewCopyleftStrength::Copyleft, NewCopyleftStrength::Copyleft) => false, // 相同Copyleft通常不兼容
            (NewCopyleftStrength::Copyleft, NewCopyleftStrength::SourceAvailable) |
            (NewCopyleftStrength::SourceAvailable, NewCopyleftStrength::Copyleft) => false,

            // 🚨 最高风险 - 不兼容
            (NewCopyleftStrength::Commercial, _) | (_, NewCopyleftStrength::Commercial) => false,
            (NewCopyleftStrength::UnstatedLicense, _) | (_, NewCopyleftStrength::UnstatedLicense) => false,

            // 其他组合需要特殊处理
            _ => self.check_specific_compatibility(a, b),
        }
    }

    fn check_specific_compatibility(&self, a: &NewLicense, b: &NewLicense) -> bool {
        // Handle specific license compatibility based on actual SPDX identifiers
        match (a.id.as_str(), b.id.as_str()) {
            // GPL版本兼容性
            ("GPL-2.0-only", id) if id.contains("GPL-3.0") => false,
            (id, "GPL-2.0-only") if id.contains("GPL-3.0") => false,
            ("GPL-2.0-or-later", id) if id.contains("GPL-3.0") => true,
            (id, "GPL-2.0-or-later") if id.contains("GPL-3.0") => true,

            // LGPL与GPL兼容性
            (id1, id2) if id1.contains("LGPL-3.0") && id2.contains("GPL-3.0") => true,
            (id1, id2) if id1.contains("GPL-3.0") && id2.contains("LGPL-3.0") => true,

            // CopyleftLimited之间的兼容性
            ("LGPL-2.1-only", "LGPL-2.1-or-later") => true,
            ("LGPL-2.1-or-later", "LGPL-2.1-only") => true,
            ("LGPL-3.0-only", "LGPL-3.0-or-later") => true,
            ("LGPL-3.0-or-later", "LGPL-3.0-only") => true,

            // Permissive与CopyleftLimited
            ("MIT", "LGPL-2.1") | ("LGPL-2.1", "MIT") => true,
            ("MIT", "LGPL-3.0") | ("LGPL-3.0", "MIT") => true,
            ("Apache-2.0", "LGPL-3.0") | ("LGPL-3.0", "Apache-2.0") => true,

            // Public Domain兼容性
            ("CC0-1.0", _) | (_, "CC0-1.0") => true,
            ("Unlicense", _) | (_, "Unlicense") => true,

            // 相同许可证家族
            (id1, id2) if self.same_license_family(id1, id2) => true,

            // 默认策略：保守处理未知组合
            _ => {
                // 对于未知组合，基于风险等级判断
                let a_order = crate::models::new_copyleft_strength_order(&a.copyleft_strength);
                let b_order = crate::models::new_copyleft_strength_order(&b.copyleft_strength);
                
                // 如果两个都是中等风险以上，视为不兼容
                a_order <= 5 && b_order <= 5
            }
        }
    }

    fn same_license_family(&self, id1: &str, id2: &str) -> bool {
        let families = [
            ("MIT", vec!["MIT", "Expat", "X11"]),
            ("BSD", vec!["BSD-2-Clause", "BSD-3-Clause", "BSD-4-Clause"]),
            ("Apache", vec!["Apache-1.1", "Apache-2.0"]),
            ("GPL", vec!["GPL-2.0", "GPL-3.0", "GPL-2.0-only", "GPL-3.0-only"]),
            ("LGPL", vec!["LGPL-2.0", "LGPL-2.1", "LGPL-3.0", "LGPL-2.1-only", "LGPL-3.0-only"]),
        ];

        for (family, members) in families.iter() {
            let id1_in = members.iter().any(|m| id1.contains(m));
            let id2_in = members.iter().any(|m| id2.contains(m));
            if id1_in && id2_in {
                return true;
            }
        }
        false
    }

    fn choose_stronger_license(&self, a: &NewLicense, b: &NewLicense) -> NewLicense {
        let a_strength = crate::models::new_copyleft_strength_order(&a.copyleft_strength);
        let b_strength = crate::models::new_copyleft_strength_order(&b.copyleft_strength);

        if a_strength >= b_strength {
            a.clone()
        } else {
            b.clone()
        }
    }

    fn find_strongest_copyleft(&self, licenses: &[NewLicense]) -> NewCopyleftStrength {
        licenses.iter()
            .map(|l| &l.copyleft_strength)
            .max_by_key(|s| crate::models::new_copyleft_strength_order(s))
            .unwrap_or(&NewCopyleftStrength::PublicDomain)
            .clone()
    }

    fn choose_recommended_license(&self, licenses: &[NewLicense]) -> Option<NewLicense> {
        if licenses.is_empty() {
            return None;
        }

        // Prefer permissive licenses, then weak copyleft, then strong copyleft
        let mut sorted_licenses = licenses.to_vec();
        sorted_licenses.sort_by_key(|l| crate::models::new_copyleft_strength_order(&l.copyleft_strength));

        sorted_licenses.into_iter().next()
    }

    fn assess_risk_level(&self, strongest: &NewCopyleftStrength, licenses: &[NewLicense]) -> RiskLevel {
        if licenses.is_empty() {
            return RiskLevel::Critical;
        }

        match strongest {
            NewCopyleftStrength::PublicDomain | NewCopyleftStrength::Permissive => RiskLevel::Low,
            NewCopyleftStrength::CopyleftLimited => RiskLevel::Medium,
            NewCopyleftStrength::Copyleft => RiskLevel::High,
            NewCopyleftStrength::UnstatedLicense => RiskLevel::Unknown,
            _ => RiskLevel::Medium, // Handle other cases
        }
    }

    fn generate_compliance_notes(&self, licenses: &[NewLicense], recommended: &Option<NewLicense>) -> Vec<String> {
        let mut notes = Vec::new();

        if licenses.is_empty() {
            notes.push("No compatible licenses found - this is a licensing conflict!".to_string());
            return notes;
        }

        if let Some(rec) = recommended {
            notes.push(format!("Recommended license choice: {}", rec.id));

            match rec.copyleft_strength {
                NewCopyleftStrength::Copyleft => {
                    notes.push("🔴 Copyleft: All derivative works must use compatible licenses".to_string());
                    notes.push("Required: Provide complete source code upon distribution".to_string());
                    notes.push("Caution: Static linking may affect entire codebase".to_string());
                }
                NewCopyleftStrength::CopyleftLimited => {
                    notes.push("🟡 CopyleftLimited: Only modifications to this component must be open-sourced".to_string());
                    notes.push("Dynamic linking generally acceptable".to_string());
                }
                NewCopyleftStrength::Permissive | NewCopyleftStrength::PublicDomain => {
                    notes.push("✅ Permissive/PublicDomain: Minimal compliance requirements".to_string());
                    notes.push("Required: Include license notice and attribution".to_string());
                }
                NewCopyleftStrength::UnstatedLicense => {
                    notes.push("Unknown license: Manual legal review required".to_string());
                }
                _ => {
                    notes.push("Special license type: Review specific requirements".to_string());
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

    fn find_conflicts(&self, licenses: &[NewLicense]) -> Vec<String> {
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