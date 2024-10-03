use clap::{Parser, ValueHint};
use json_value::JsonValue;
use std::fs::read_to_string;

pub mod error;
pub mod json_value;
pub mod lexer;
pub mod parser;

#[derive(Parser, Debug)]
#[command(name = "JSON Parser")]
#[command(version = "0.1.0")]
#[command(about = "Parses and manipulates JSON data")]
struct Cli {
    /// Input JSON file
    #[arg(value_hint = ValueHint::FilePath)]
    input: String,

    /// Key to query
    #[arg(required = false)]
    key: Option<String>,

    /// Key-value to set (format: key=value)
    #[arg(short, long, value_hint = ValueHint::Other)]
    set: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let file_contents = read_to_string(&cli.input).expect("Something went wrong reading the file");
    let lexer = lexer::Lexer::new(&file_contents);
    let mut parser = parser::Parser::new(lexer);

    let mut json_value = parser.parse().expect("Failed to parse JSON");

    // Query a specific key
    if let Some(key) = cli.key {
        if let Some(value) = json_value.get(&key) {
            println!("Value for key '{}': {}", key, value);
        } else {
            println!("Key '{}' not found", key);
        }
    }

    // Set a key-value pair
    if let Some(set) = cli.set {
        let parts: Vec<&str> = set.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].to_string();
            let value = parts[1].to_string(); // You can enhance this to parse values appropriately
            json_value.set(key, JsonValue::String(value));
            println!("Updated JSON: {}", json_value.to_json());
        } else {
            println!("Invalid format for set. Use key=value");
        }
    }
}
