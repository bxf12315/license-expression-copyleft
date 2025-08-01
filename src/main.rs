use clap::Parser;
use parser::license_expression_parser::LicenseExpressionParser;

#[derive(Parser)]
#[command(name = "license-expression-copyleft")]
#[command(about = "Analyze license expressions for copyleft strength and compatibility")]
struct Args {
    /// The license expression to analyze
    #[arg(value_name = "LICENSE_EXPRESSION")]
    license_expression: String,
}

fn main() {
    env_logger::init();
    
    let args = Args::parse();
    let parser = LicenseExpressionParser::new();
    
    let result = parser.analyze(&args.license_expression);
    println!("{}", result);
}
