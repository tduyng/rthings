use serde::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

#[derive(Serialize)]
struct Entry {
    index: usize,
    code: String,
    name: String,
}

fn parse_tab_file(path: &str, output_path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries: Vec<Entry> = Vec::new();
    let mut line_count = 0;
    let mut code_index: Option<usize> = None;
    let mut name_index: Option<usize> = None;

    for line_result in reader.lines() {
        // Added ? for error handling
        line_count += 1;
        let line = line_result?;

        // Parse header line only once outside the loop
        if line_count == 1 {
            let headers: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();
            code_index = headers.iter().position(|h| h == "Id");
            name_index = headers.iter().position(|h| h == "Ref_Name");
            continue; // Skip remaining code for the header line
        }

        let parts: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();

        if parts.is_empty() {
            continue;
        }

        // Use cached indices for performance and handle potential None values
        let code = if let Some(index) = code_index {
            parts.get(index).unwrap_or(&"".to_string()).to_string()
        } else {
            "".to_string()
        };

        let name = if let Some(index) = name_index {
            parts.get(index).unwrap_or(&"".to_string()).to_string()
        } else {
            "".to_string()
        };

        if !code.is_empty() && !name.is_empty() {
            let entry = Entry {
                index: line_count - 2, // Adjust index to start from 0
                code,
                name,
            };
            entries.push(entry);
        }
    }

    let json_data = serde_json::to_string(&entries)?;

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_path)?;
    output_file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn main() {
    let tab_path = "src/iso6393.tab";
    let output_path = "src/output.json";

    match parse_tab_file(tab_path, output_path) {
        Ok(_) => println!("Successfully parsed data and wrote to JSON file"),
        Err(err) => println!("Error parsing file: {}", err),
    }
}
