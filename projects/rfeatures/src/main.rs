use glob::glob;
use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let root_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    if !root_dir.exists() {
        eprintln!("Error: The directory {:?} does not exist.", root_dir);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

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
    let content = fs::read_to_string(path)?;
    let mut modified_content = String::new();

    let start_pattern =
        Regex::new(r"^\s*And response json body should match snapshot\s*$").unwrap();
    let scenario_pattern = Regex::new(r"^\s*Scenario:\s*").unwrap();

    let mut inside_target_block = false;
    let mut modified = false;

    println!("Processing file: {:?}", path);

    for line in content.lines() {
        if start_pattern.is_match(line) && !inside_target_block {
            modified_content.push_str("And response body should match snapshot\n");
            inside_target_block = true;
            modified = true;
        } else if inside_target_block && scenario_pattern.is_match(line) {
            inside_target_block = false;
            modified_content.push_str(line);
            modified_content.push('\n');
        } else if inside_target_block {
            continue;
        } else {
            modified_content.push_str(line);
            modified_content.push('\n');
        }
    }

    if modified {
        println!("File modified: {:?}", path);
        let mut file = fs::File::create(path)?;
        file.write_all(modified_content.as_bytes())?;
    } else {
        println!("No changes needed for file: {:?}", path);
    }

    Ok(())
}
