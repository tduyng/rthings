use csv::ReaderBuilder;
use inflector::Inflector;
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/data.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut json_objects = Vec::new();
    let headers = rdr.headers()?.clone();

    // Iterate through each record in the CSV file
    for result in rdr.records() {
        let record = result?;

        let mut record_map = serde_json::Map::new();
        for (index, value) in record.iter().enumerate() {
            let header = headers[index].to_string();
            let camel_case_header = header.to_snake_case();
            record_map.insert(camel_case_header, Value::String(value.to_string()));
        }

        // Convert the HashMap to JSON and add it to the vector
        let json_value: Value = serde_json::to_value(&record_map)?;
        json_objects.push(json_value);
    }

    let json_output = json!(json_objects);

    // Write to JSON file
    let mut output_file = File::create("data/data.json")?;
    output_file.write_all(serde_json::to_string_pretty(&json_output)?.as_bytes())?;

    println!("{}", serde_json::to_string_pretty(&json_output)?);

    Ok(())
}
