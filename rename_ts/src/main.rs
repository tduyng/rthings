use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required number of arguments is provided
    if args.len() != 3 {
        println!("Usage: {} <folder_path> <file_extension>", args[0]);
        return;
    }

    // Extract the folder path and file extension from the arguments
    let folder_path = &args[1];
    let file_extension = &args[2];

    // Read directory
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Check if the file has the expected extension
            if file_name_str.ends_with(file_extension) {
                // Extract the folder name
                let folder_name = file_name_str.trim_end_matches(file_extension);

                // Create directory if it doesn't exist
                let dir_path = format!("{}/{}", folder_path, folder_name);
                if !Path::new(&dir_path).exists() {
                    fs::create_dir(&dir_path).expect("Failed to create directory");
                }

                // Move the file
                let new_file_path = format!("{}/index{}", dir_path, file_extension);
                fs::rename(entry.path(), new_file_path).expect("Failed to move file");
            }
        }
    }
}
