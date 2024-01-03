use crate::{android::fetch_emulators, ios::fetch_ios_simulators};
#[derive(Debug)]
pub enum Platform {
    IOS,
    ANDROID,
}
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub identifier: String,
    pub booted: bool,
    pub platform: Platform,
}

pub fn str_to_platform(input: &str) -> Option<Platform> {
    match input.to_uppercase().as_str() {
        "IOS" => Some(Platform::IOS),
        "ANDROID" => Some(Platform::ANDROID),
        _ => None,
    }
}

pub fn fetch_devices(platform: Platform) -> Vec<Device> {
    match platform {
        Platform::ANDROID => {
            return fetch_emulators();
        }
        Platform::IOS => {
            return fetch_ios_simulators();
        }
    }
}
