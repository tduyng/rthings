use std::{fs, path::Path};

fn main() {
    let dir_path = "/Users/tien-duy.nguyen/Documents/Export"; // Specify your directory path here
    if Path::new(dir_path).is_dir() {
        process_directory(dir_path).expect("Failed to process directory");
    } else {
        println!("The provided path is not a directory.");
    }
}

fn process_directory(dir_path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_directory(path.to_str().unwrap())?;
            if let Some(new_name) = extract_text_before_id(path.file_name().unwrap().to_str().unwrap()) {
                let new_path = path.with_file_name(new_name);
                fs::rename(path, new_path)?;
            }
        } else if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            if let Some(new_name) = extract_text_before_id(filename) {
                let new_path = path.with_file_name(new_name);
                fs::rename(path, new_path)?;
            }
        }
    }
    Ok(())
}

fn extract_text_before_id(filename: &str) -> Option<String> {
    let parts: Vec<&str> = filename.split(' ').collect();
    if parts.len() > 1 {
        Some(format!("{}.md", parts[0]))
    } else {
        None
    }
}