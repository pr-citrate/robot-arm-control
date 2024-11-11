// src-tauri/serial.rs

use serde::{Deserialize, Serialize};
use serialport;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RobotState {
    pub J1: u8,
    pub J2: u8,
    pub J3: u8,
    pub J4: u8,
    pub J5: u8,
    pub J6: u8,
    pub Di1: bool,
    pub Di2: bool,
    pub Di3: bool,
    pub Do1: bool,
    pub Do2: bool,
    pub Do3: bool,
    pub robotSpeed: u8,
}

pub struct SerialPortManager {
    port: Arc<Mutex<Option<Box<dyn serialport::SerialPort + Send>>>>,
}

impl SerialPortManager {
    pub fn new() -> Self {
        Self {
            port: Arc::new(Mutex::new(None)),
        }
    }

    pub fn initialize(&self, port_name: &str, baud_rate: u32) -> Result<(), serialport::Error> {
        let s = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()?;
        let mut port_lock = self.port.lock().unwrap();
        *port_lock = Some(s);
        Ok(())
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), serialport::Error> {
        let mut port_lock = self.port.lock().unwrap(); // Declare as mutable
        if let Some(ref mut port) = *port_lock {
            port.write_all(data)?;
            Ok(())
        } else {
            Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::Other),
                "Serial port not initialized",
            ))
        }
    }

    pub fn read_data(&self) -> Result<RobotState, serialport::Error> {
        let mut port_lock = self.port.lock().unwrap(); // Declare as mutable
        if let Some(ref mut port) = *port_lock {
            let mut buffer: Vec<u8> = vec![0; 15];
            match port.read_exact(&mut buffer) {
                Ok(_) => {
                    if buffer.len() != 15 || buffer[0] != 253 || buffer[14] != 254 {
                        return Err(serialport::Error::new(
                            serialport::ErrorKind::InvalidInput,
                            "Invalid data packet",
                        ));
                    }
                    Ok(RobotState {
                        J1: buffer[1],
                        J2: buffer[2],
                        J3: buffer[3],
                        J4: buffer[4],
                        J5: buffer[5],
                        J6: buffer[6],
                        Di1: buffer[7] != 0,
                        Di2: buffer[8] != 0,
                        Di3: buffer[9] != 0,
                        Do1: buffer[10] != 0,
                        Do2: buffer[11] != 0,
                        Do3: buffer[12] != 0,
                        robotSpeed: buffer[13],
                    })
                }
                Err(e) => Err(e.into()),
            }
        } else {
            Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::Other),
                "Serial port not initialized",
            ))
        }
    }

    pub fn list_ports() -> Result<Vec<serialport::SerialPortInfo>, serialport::Error> {
        serialport::available_ports()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub serial_manager: Arc<SerialPortManager>,
}

#[tauri::command]
pub fn list_serial_ports() -> Result<Vec<String>, String> {
    match SerialPortManager::list_ports() {
        Ok(ports) => {
            let port_names = ports
                .into_iter()
                .map(|port| port.port_name)
                .collect::<Vec<String>>();
            Ok(port_names)
        }
        Err(e) => Err(format!("Failed to list serial ports: {}", e)),
    }
}

#[tauri::command]
pub fn initialize_serial(
    state: State<'_, AppState>,
    port: String,
    baud_rate: u32,
) -> Result<String, String> {
    match state.serial_manager.initialize(&port, baud_rate) {
        Ok(_) => Ok("Serial port initialized".into()),
        Err(e) => Err(format!("Failed to open serial port: {}", e)),
    }
}

#[tauri::command]
pub fn send_robot_commands(
    state: State<'_, AppState>,
    robot_state: RobotState,
) -> Result<(), String> {
    let mut data = [0u8; 15];
    data[0] = 253;
    data[1] = robot_state.J1;
    data[2] = robot_state.J2;
    data[3] = robot_state.J3;
    data[4] = robot_state.J4;
    data[5] = robot_state.J5;
    data[6] = robot_state.J6;
    data[7] = robot_state.Di1 as u8;
    data[8] = robot_state.Di2 as u8;
    data[9] = robot_state.Di3 as u8;
    data[10] = robot_state.Do1 as u8;
    data[11] = robot_state.Do2 as u8;
    data[12] = robot_state.Do3 as u8;
    data[13] = robot_state.robotSpeed;
    data[14] = 254;

    state
        .serial_manager
        .send_data(&data)
        .map_err(|e| format!("Failed to send data: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn read_robot_state(state: State<'_, AppState>) -> Result<RobotState, String> {
    state
        .serial_manager
        .read_data()
        .map_err(|e| format!("Failed to read data: {}", e))
}
