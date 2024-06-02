use clap::Parser;
use std::error::Error;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "tubed",
    version = "1.0",
    author = "Z <me@tduyng.com>",
    about = "Downloads YouTube videos"
)]
struct Cli {
    #[clap(short, long, help = "The YouTube URL to download")]
    url: String,

    #[clap(short, long, help = "Output format: mp4 or mp3", default_value = "mp4")]
    format: Format,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Format {
    Mp4,
    Mp3,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match download_video(&args.url, &args.format) {
        Ok(_) => println!("Download and conversion completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn download_video(url: &str, format: &Format) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("yt-dlp");
    cmd.arg(url);

    match format {
        Format::Mp4 => cmd
            .arg("--format")
            .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4"),
        Format::Mp3 => cmd.arg("--extract-audio").arg("--audio-format").arg("mp3"),
    };

    let output = cmd.output()?;

    if output.status.success() {
        println!("Download completed successfully.");
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("yt-dlp failed: {}", error_message),
        )))
    }
}
