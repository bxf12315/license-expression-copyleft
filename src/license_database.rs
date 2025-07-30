use std::collections::HashMap;
use crate::models::{License, CopyleftStrength};

pub fn init_license_database() -> HashMap<String, License> {
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
        license_db.insert(id.to_string(), License {
            id: id.to_string(),
            name: name.to_string(),
            copyleft_strength: strength,
            is_osi_approved: osi,
        });
    }
    license_db
}