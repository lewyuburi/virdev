use std::env;

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use crate::{
    android::launch_emulator,
    device::{str_to_platform, Device, Platform},
};

pub fn default_tray(ios_devices: Vec<Device>, android_devices: Vec<Device>) -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let android_emu_label =
        CustomMenuItem::new("android_label".to_string(), "Android emulators").disabled();

    let mut tray_menu = SystemTrayMenu::new();

    tray_menu = tray_menu.add_item(android_emu_label);

    tray_menu = tray_menu_device_options(android_devices, tray_menu);

    if env::consts::OS == "macos" {
        let ios_emu_label =
            CustomMenuItem::new("iOS_label".to_string(), "iOS simulators").disabled();

        tray_menu = tray_menu
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(ios_emu_label);

        tray_menu = tray_menu_device_options(ios_devices, tray_menu);
    }

    tray_menu = tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("VirDev");

    return system_tray;
}

pub fn tray_event_handler() -> impl Fn(&tauri::AppHandle, SystemTrayEvent) {
    |app, event| match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {
                let parts: Vec<&str> = id.split('/').collect();

                let platform = &parts[0];
                let device_id = &parts[1];

                if let Some(platform) = str_to_platform(platform) {
                    match platform {
                        Platform::ANDROID => {
                            launch_emulator(device_id.to_string());
                        }
                        Platform::IOS => {}
                    }
                } else {
                    println!("Invalid platform string: {}", platform);
                }
            }
        },
        _ => {}
    }
}

fn tray_menu_device_options(devices: Vec<Device>, tray_menu: SystemTrayMenu) -> SystemTrayMenu {
    return devices.iter().fold(tray_menu, |menu, avd| {
        menu.add_item(CustomMenuItem::new(
            format!("{:?}/{}", avd.platform, avd.identifier.clone()),
            avd.name.clone(),
        ))
    });
}
