// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod android;
mod device;
mod ios;
mod tray;

use std::env;

use crate::{
    device::{fetch_devices, Device, Platform},
    tray::{default_tray, tray_event_handler},
};

fn main() {
    let mut android_emulators: Vec<Device> = Vec::new();
    let mut ios_simulators: Vec<Device> = Vec::new();

    if env::consts::OS == "macos" {
        ios_simulators = fetch_devices(Platform::IOS);
    }

    android_emulators = fetch_devices(Platform::ANDROID);

    let system_tray = default_tray(ios_simulators, android_emulators);

    tauri::Builder::default()
        .setup(|app| Ok(app.set_activation_policy(tauri::ActivationPolicy::Accessory)))
        .system_tray(system_tray)
        .on_system_tray_event(tray_event_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
