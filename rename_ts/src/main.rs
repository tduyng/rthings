use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Move and rename Typescript files", long_about = None)]
struct Cli {
    #[arg(long)]
    folder: String,
    #[arg(long)]
    ext: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = process_files(&cli.folder, &cli.ext) {
        eprintln!("Error: {}", err);
    }
}

fn process_files(folder_path: &str, file_extension: &str) -> Result<(), String> {
    // Read directory
    let entries =
        fs::read_dir(folder_path).map_err(|e| format!("Failed to read directory: {}", e))?;

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
                fs::create_dir(&dir_path)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }

            // Move the file
            let new_file_path = format!("{}/index{}", dir_path, file_extension);
            fs::rename(entry.path(), &new_file_path)
                .map_err(|e| format!("Failed to move file: {}", e))?;
        }
    }

    Ok(())
}
