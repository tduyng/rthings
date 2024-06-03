use std::{io, process::Command};

fn main() -> io::Result<()> {
    let networks = list_network_interfaces()?;
    println!("Network interfaces: {:?}", networks);

    if let Some(interface) = networks.first() {
        println!("Testing network interface: {}", interface);
        let (download_speed, upload_speed) = test_network_speed(interface)?;
        println!("Download speed: {:.2} Mbps", download_speed);
        println!("Upload speed: {:.2} Mbps", upload_speed);
    } else {
        println!("No network interfaces found.");
    }

    Ok(())
}

pub fn list_network_interfaces() -> io::Result<Vec<String>> {
    let output = Command::new("networksetup")
        .args(&["-listallhardwareports"])
        .output()?;

    let stdout = String::from_utf8(output.stdout).unwrap();
    let interfaces = stdout
        .lines()
        .filter(|line| line.starts_with("Device:"))
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(interface) = parts.get(1) {
                interface.to_string()
            } else {
                String::new()
            }
        })
        .collect();

    Ok(interfaces)
}

fn test_network_speed(interface: &str) -> io::Result<(f64, f64)> {
    // Placeholder for actual speed test implementation
    // For now, just return dummy values
    Ok((10.0, 5.0)) // Dummy values for download and upload speeds
}
