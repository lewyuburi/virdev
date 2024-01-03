use std::process::Command;

use crate::device::{Device, Platform};

fn get_android_home_path() -> String {
    let mut android_home_path: String = "".to_string();

    if let Some(android_home) = std::env::var("ANDROID_HOME").ok().filter(|s| !s.is_empty()) {
        android_home_path = android_home;
    }

    return android_home_path;
}

fn get_emulator_path() -> String {
    return format!(
        "{}/{}",
        get_android_home_path(),
        "emulator/emulator".to_string()
    );
}

pub fn fetch_emulators() -> Vec<Device> {
    let emulator_path = get_emulator_path();

    let output = Command::new(emulator_path)
        .args(&["-list-avds"])
        .output()
        .expect("Failed to run avdmanager");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return output_str
            .lines()
            .map(|s| s.to_string())
            .map(|device| Device {
                name: device.to_string(),
                identifier: device.to_string(),
                booted: false,
                platform: Platform::ANDROID,
            })
            .collect();
    } else {
        eprintln!("Error running avdmanager: {:?}", output.status);
        return Vec::new();
    }
}

pub fn launch_emulator(id: String) {
    let emulator_path = get_emulator_path();

    println!("Starting Android emulator: id {}", id);

    let _ = Command::new(emulator_path)
        .args(&["-avd", &id])
        .spawn()
        .expect(&format!("Failed to start Android emulator: id {}", id));
}
