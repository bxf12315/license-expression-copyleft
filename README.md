# License Expression Copyleft Analyzer

A copyleft strength analysis tool for license expressions based on Scancode License DB.

## 1. Running Requirements and Environment Setup

### System Requirements
- **Rust**: Requires Rust 1.70 or higher
- **Operating System**: Supports Linux, macOS, Windows

### Installation Steps
1. **Install Rust** (if not already installed):
   ```bash
   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Windows
   # Download and run rustup-init.exe from https://rustup.rs/
   ```

2. **Clone the project**:
   ```bash
   git clone <repository-url>
   cd license-expression-copyleft
   ```

3. **Build the project**:
   ```bash
   cargo build --release
   ```

4. **Run tests**:
   ```bash
   cargo run -- "MIT OR Apache-2.0"
   ```

## 2. Copyleft Definition Based on Scancode License DB

This project uses the license classification standards from [Scancode License Database](https://github.com/nexB/scancode-toolkit), categorizing licenses by copyleft strength into the following levels:

### License Categories
- **CLA**: Contributor License Agreement - describes contribution acceptance rules and licensing terms for ongoing development
- **Commercial**: Third-party proprietary software offered under direct commercial license between supplier and customer
- **Copyleft**: Strong copyleft licenses requiring derivative works to use the same license (e.g., GPL-2.0, GPL-3.0)
- **Copyleft Limited**: Limited copyleft requiring source code redistribution with attribution, but with license-specific limitations (e.g., LGPL-2.1, LGPL-3.0)
- **Free Restricted**: Permissive-style licenses with usage or redistribution restrictions (e.g., non-commercial use only)
- **Patent License**: Licenses that apply to patents rather than specific software, may be used with other software licenses
- **Permissive**: Non-copyleft open source licenses requiring attribution but allowing proprietary use (e.g., MIT, BSD, Apache-2.0)
- **Proprietary Free**: Proprietary software that may not require commercial license but has specific terms and conditions
- **Public Domain**: Software available without explicit obligations but requiring license notice retention per organization policy
- **Source-available**: Software with source code available for viewing and limited modification, but not meeting open-source criteria
- **Unstated License**: Third-party software with copyright notice but no stated license, requiring fact-finding with copyright owner

### Data Sources
- License database file: `index.json`
- Contains complete definitions for 800+ open source licenses
- Based on authoritative classification standards from Scancode Toolkit

## 3. Analyze Result Explanation

### Output Fields Explanation
After running the analysis command, the output contains the following information:

#### Basic Information
- **Original Expression**: The original license expression input
- **Risk Level**: Overall risk level (Low/Medium/High/Critical)
- **Strongest Copyleft**: The strongest copyleft type in the expression

#### Parse Results
- **Parsed Expression**: The parsed structured expression
- **Possible Licenses**: All possible license options
- **Recommended Choice**: The recommended license choice

#### Compliance Recommendations
- **Compliance Notes**: Specific compliance considerations
- **Alternative Licenses**: Available alternative licenses
- **Manual Review**: Manual review requirements

### Risk Level Mapping
- **Low**: No copyleft or permissive licenses
- **Medium**: Limited copyleft requirements
- **High**: Strong copyleft requirements
- **Critical**: Commercial restrictions or unknown licenses

## 4. Copyleft Determination Logic for AND/OR/WITH Operations

### Operator Explanation

#### AND Operator
- **Rule**: Adopts the strictest copyleft requirement
- **Example**: `GPL-3.0 AND MIT` → Results in GPL-3.0's strong copyleft
- **Logic**: Must satisfy all license requirements, so choose the strictest

#### OR Operator
- **Rule**: Adopts the most permissive copyleft option
- **Example**: `GPL-3.0 OR MIT` → Can choose MIT (no copyleft)
- **Logic**: Can choose any license, recommend the most permissive

#### WITH Operator (Exception Clause)
- **Rule**: Inherits the copyleft strength of the base license
- **Example**: `GPL-3.0 WITH GCC-exception-3.0` → Still GPL-3.0 strength
- **Logic**: Exception clauses do not affect base copyleft requirements

### Complex Expression Handling

#### Nested Expressions
```
(GPL-2.0 OR Apache-2.0) AND MIT
```
- First process OR: Can choose Apache-2.0 (permissive)
- Then process AND: Apache-2.0 AND MIT → Choose the stricter one

#### Multi-level Nesting
```
((LGPL-2.1 OR GPL-3.0) AND MIT) OR BSD-3-Clause
```
- Parse from innermost to outermost
- Ultimately choose the combination that best meets requirements

### Usage Examples

#### Simple Analysis
```bash
cargo run -- "MIT OR Apache-2.0"
```

#### Complex Expression
```bash
cargo run -- "(GPL-2.0 OR LGPL-3.0) AND Apache-2.0"
```

#### With Exception Clause
```bash
cargo run -- "GPL-3.0 WITH Classpath-exception-2.0"
```

## License

This project uses the MIT license, see [LICENSE](LICENSE) file for details.