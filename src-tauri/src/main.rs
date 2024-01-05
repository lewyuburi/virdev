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
    let mut ios_simulators: Vec<Device> = Vec::new();

    if env::consts::OS == "macos" {
        ios_simulators = fetch_devices(Platform::IOS);
    }

    let system_tray = default_tray(ios_simulators, fetch_devices(Platform::ANDROID));

    let mut app = tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(tray_event_handler())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    if env::consts::OS == "macos" {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    }

    app.run(|_app_handle, _event| {});
}
