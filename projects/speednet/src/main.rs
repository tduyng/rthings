use std::{io, process::Command};

fn main() {
    let networks = list_network_interfaces().unwrap();
    println!("Network interfaces: {:?}", networks);
}

pub fn list_network_interfaces() -> io::Result<Vec<String>> {
    let output = Command::new("networksetup")
        .args(&["-listallhardwareports"])
        .output()?;

    let stdout = String::from_utf8(output.stdout).unwrap();
    let interfaces = stdout
        .lines()
        .skip_while(|line| !line.starts_with("Hardware Port:"))
        .skip(1) // Skip the "Hardware Port:" line itself
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(interfaces)
}
