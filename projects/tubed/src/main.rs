use clap::Parser;

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
    format: String,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    match download_video(&args.url, &args.format) {
        Ok(_) => println!("Download and conversion completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

fn download_video(_url: &str, _format: &str) -> Result<(), String> {
    Ok(())
}
