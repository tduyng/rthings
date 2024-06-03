use clap::Parser;
use std::error::Error;
use std::io;
use std::process::Stdio;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Parser)]
#[command(
    name = "tubed",
    version = "0.1.0",
    author = "Z <me@tduyng.com>",
    about = "Downloads YouTube videos"
)]
struct Cli {
    #[clap(long, help = "The YouTube URL to download")]
    url: String,

    #[clap(
        short,
        long,
        help = "Output format (e.g., mp4, mp3, best)",
        default_value = "mp4"
    )]
    format: String,

    #[clap(
        short,
        long,
        help = "Output directory for the downloaded files",
        default_value = "."
    )]
    output: String,

    #[clap(long, help = "Additional yt-dlp options")]
    options: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let start_time = Instant::now();
    match download_video(&args).await {
        Ok(_) => {
            let duration = start_time.elapsed();
            println!(
                "Download and conversion completed successfully in {:.2?}.",
                duration
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

async fn download_video(args: &Cli) -> Result<(), Box<dyn Error>> {
    let output_options: Vec<&str> = match args.format.as_str() {
        "mp4" => vec!["--format", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/mp4"],
        "mp3" => vec!["--extract-audio", "--audio-format", "mp3"],
        _ => vec!["--format", "best"],
    };

    let mut command_args = output_options;

    // Add progress output
    command_args.push("--progress");

    // Custom output path
    let output_path = format!("{}/%(title)s.%(ext)s", &args.output);
    command_args.push("--output");
    command_args.push(&output_path);

    // Add URL
    command_args.push(args.url.as_str());

    // Add additional options if any
    if let Some(opts) = &args.options {
        command_args.extend(opts.split_whitespace());
    }

    let mut child = Command::new("yt-dlp")
        .args(&command_args)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        println!("{}", line);
    }

    let status = child.wait().await?;
    if status.success() {
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "yt-dlp failed to download the video",
        )))
    }
}
