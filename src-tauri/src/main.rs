// src-tauri/src/main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod serial;

use serial::SerialPortManager;
use serial::{
    initialize_serial, list_serial_ports, read_robot_state, send_robot_commands, AppState,
};
use std::sync::Arc;

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            serial_manager: Arc::new(SerialPortManager::new()),
        })
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            initialize_serial,
            send_robot_commands,
            read_robot_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
