use std::process::Command;

use regex::Regex;

use crate::device::{Device, Platform};

pub fn fetch_ios_simulators() -> Vec<Device> {
    let simulator_path = "/usr/bin/xcrun";

    let output = Command::new(simulator_path)
        .args(&["simctl", "list", "devices", "available"])
        .output()
        .expect("Failed to run avdmanager");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return parse_ios_devices(&output_str);
    } else {
        eprintln!("Error running avdmanager: {:?}", output.status);
        Vec::new()
    }
}

fn parse_ios_devices(result: &str) -> Vec<Device> {
    let mut devices: Vec<Device> = Vec::new();
    let mut os_version = String::new();

    let current_os_regex = Regex::new(r"-- (.*?) --").unwrap();
    let device_regex = Regex::new(r"(.*?) (\(([0-9.]+)\) )?\(([0-9A-F-]+)\) (\(.*?)\)").unwrap();

    for line in result.lines() {
        if let Some(captures) = current_os_regex.captures(line) {
            if let Some(current_os) = captures.get(1) {
                os_version = current_os.as_str().to_string();
            }
        }

        if let Some(captures) = device_regex.captures(line) {
            let name = captures.get(1).map_or("", |m| m.as_str()).trim();
            let identifier = captures.get(4).map_or("", |m| m.as_str());
            let booted = captures
                .get(6)
                .map_or("", |m| m.as_str())
                .contains("Booted");
            let platform = Platform::IOS;

            devices.push(Device {
                name: format!("[{}] - {}", os_version, name.to_string()),
                identifier: identifier.to_string(),
                booted: booted,
                platform,
            });
        }
    }

    devices
}
