use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::write::ZipWriter;
use zip::CompressionMethod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = "docs";

    // Create the zip file
    let file_zip = File::create("archive.zip")?;
    let mut zip = ZipWriter::new(file_zip);

    compress_folder(folder_path, &mut zip)?;

    zip.finish()?;
    Ok(())
}

fn compress_folder<P: AsRef<Path>>(
    folder_path: P,
    zip_writer: &mut ZipWriter<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = folder_path.as_ref();
    let options = FileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755);

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let mut file = File::open(&path)?;
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content)?;

            zip_writer.start_file(file_name, options)?;
            zip_writer.write_all(&file_content)?;
        }
    }

    Ok(())
}
