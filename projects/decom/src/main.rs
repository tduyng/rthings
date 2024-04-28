use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    decompress_archive("archive.zip", "decompressed")?;
    Ok(())
}

fn decompress_archive(
    zip_path: &str,
    output_folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the Zip archive
    let file_zip = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file_zip)?;

    // Create the output folder if it doesn't exist
    if !Path::new(output_folder).exists() {
        fs::create_dir_all(output_folder)?;
    }

    // Iterate through each file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        // Get the name of the file
        let file_name = file.mangled_name();

        // Create the output file path
        let mut output_file_path = PathBuf::new();
        output_file_path.push(output_folder);
        output_file_path.push(file_name);

        // Create the output file
        let mut output_file = File::create(&output_file_path)?;

        // Copy the file contents to the output file
        io::copy(&mut file, &mut output_file)?;
    }

    Ok(())
}
