use glob::glob;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // Define the root directory where .feature files are located
    let home_dir = dirs::home_dir().expect("Failed to find the home directory");
    let root_dir = home_dir.join("Desktop/git-projects/ekino/hodor/tests");

    // Check if the directory exists
    if !root_dir.exists() {
        eprintln!("Error: The directory {:?} does not exist.", root_dir);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    // Use glob to find all .feature files recursively
    let pattern = format!("{}/**/*.feature", root_dir.display());
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Err(e) = process_file(&path) {
                    eprintln!("Error processing file {:?}: {}", path, e);
                }
            }
            Err(e) => eprintln!("Error reading path: {}", e),
        }
    }

    Ok(())
}

fn process_file(path: &PathBuf) -> io::Result<()> {
    // Read the file content
    let content = fs::read_to_string(path)?;
    let mut modified_content = String::new();

    // Define regex patterns
    let start_pattern =
        Regex::new(r"^\s*And response json body should match snapshot\s*$").unwrap();
    let scenario_pattern = Regex::new(r"^\s*Scenario:\s*").unwrap();

    let mut inside_target_block = false;
    let mut modified = false;

    println!("Processing file: {:?}", path);

    // Process the file line by line
    for line in content.lines() {
        if start_pattern.is_match(line) && !inside_target_block {
            // Start of the block to replace
            modified_content.push_str("And response body should match snapshot\n");
            inside_target_block = true;
            modified = true;
        } else if inside_target_block && scenario_pattern.is_match(line) {
            // End of the block (new scenario starts)
            inside_target_block = false;
            modified_content.push_str(line);
            modified_content.push('\n');
        } else if inside_target_block {
            // Skip lines within the block (including tables)
            continue;
        } else {
            // Add non-target lines unchanged
            modified_content.push_str(line);
            modified_content.push('\n');
        }
    }

    if modified {
        println!("File modified: {:?}", path);
        // Write the modified content back to the file
        let mut file = fs::File::create(path)?;
        file.write_all(modified_content.as_bytes())?;
    } else {
        println!("No changes needed for file: {:?}", path);
    }

    Ok(())
}

