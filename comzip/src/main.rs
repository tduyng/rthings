use std::fs::File;
use std::io::{Read, Write};
use zip::write::FileOptions;
use zip::write::ZipWriter;
use zip::CompressionMethod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open("docs/lorem.txt")?;

    // Read file content
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Create the zip file
    let file_zip = File::create("archive.zip")?;
    let mut zip = ZipWriter::new(file_zip);

    // Define compression options
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755); // Adjust permissions as needed

    // Start adding file to the zip
    zip.start_file("lorem.txt", options)?;

    // Write the file content to the zip
    zip.write_all(&file_content)?;

    // Finish writing the zip
    zip.finish()?;

    Ok(())
}
