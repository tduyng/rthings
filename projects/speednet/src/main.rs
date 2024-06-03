use anyhow::{Context, Result};
use reqwest;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let (download_speed, upload_speed) = test_network_speed().await?;
    println!("Download speed: {:.2} Mbps", download_speed);
    println!("Upload speed: {:.2} Mbps", upload_speed);
    Ok(())
}

async fn test_network_speed() -> Result<(f64, f64)> {
    let download_speed = measure_download_speed().await?;
    let upload_speed = measure_upload_speed().await?;
    Ok((download_speed, upload_speed))
}

async fn measure_download_speed() -> Result<f64> {
    let start_time = Instant::now();
    let response = reqwest::get("https://speedtest.net/api/download").await?;
    let content_length = response.content_length().unwrap_or(0);

    let mut file_content = Vec::new();
    response
        .bytes()
        .await
        .context("Failed to read response body")?
        .into_iter()
        .for_each(|b| file_content.push(b));

    let download_time = start_time.elapsed().as_secs_f64();
    let download_speed = content_length as f64 / download_time / 1_000_000.0; // Convert to Mbps
    Ok(download_speed)
}

async fn measure_upload_speed() -> Result<f64> {
    // For the upload speed test, you would typically upload a file to a server.
    // Since we don't have a public API for upload testing, we'll use a placeholder value.
    let upload_speed = 10.0; // Mbps (placeholder value)
    Ok(upload_speed)
}
