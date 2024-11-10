#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod serial;

use serial::SerialPortManager;
use serial::{AppState, initialize_serial, send_robot_commands, read_robot_state};
//use tauri::State;
use std::sync::Arc;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize serial port here if needed
            Ok(())
        })
        .manage(AppState {
            serial_manager: Arc::new(
                SerialPortManager::new("/dev/ttyUSB0", 9600).expect("Failed to open serial port"),
            ),
        })
        .invoke_handler(tauri::generate_handler![
            initialize_serial,
            send_robot_commands,
            read_robot_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
