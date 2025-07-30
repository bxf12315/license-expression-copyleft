mod license_expression_parser;
mod models;
mod license_database;
mod license;

use license_expression_parser::LicenseExpressionParser;
use std::env;
use std::fs;
use std::path::Path;

// Import the License struct from the new module
use license::License;

fn main() {
    let parser = LicenseExpressionParser::new();

    // Test cases
    let test_expressions = vec![
        // "(GPL-2.0-or-later OR LGPL-3.0-or-later) AND GPL-3.0-or-later",
        "(GPL-2.0 OR Apache-2.0) AND (GPL-2.0-only AND GPL-3.0-only)",
        "MIT OR Apache-2.0",
        // "GPL-2.0-only AND GPL-3.0-only",
        // "LGPL-2.1-or-later WITH GCC-exception-2.0",
        // "Apache-2.0 AND (MIT OR BSD-3-Clause)",
        // "AGPL-3.0-or-later OR GPL-3.0-or-later",
        // "LicenseRef-0 AND Apache-2.0",
        // "EPL-2.0 OR Apache-2.0",
    ];

    for expr in test_expressions {
        println!("{}", parser.analyze(expr));
        println!();
    }

    // Example of loading licenses from index.json
    load_licenses_from_file();
}

/// Load and parse licenses from the index.json file
fn load_licenses_from_file() {
    let file_path = Path::new("index.json");
    match fs::read_to_string(file_path) {
        Ok(data) => {
            match serde_json::from_str::<Vec<License>>(&data) {
                Ok(licenses) => {
                    println!("\nSuccessfully loaded {} licenses from index.json", licenses.len());
                    // Print first few licenses as an example
                    for (i, license) in licenses.iter().take(5).enumerate() {
                        println!("  {}. License Key: {}", i + 1, license.license_key);
                    }
                },
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            }
        },
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}