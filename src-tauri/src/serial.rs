// src-tauri/serial.rs

use serde::{Deserialize, Serialize};
use serialport;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RobotState {
    pub joint_1: u8,
    pub joint_2: u8,
    pub joint_3: u8,
    pub joint_4: u8,
    pub joint_5: u8,
    pub joint_6: u8,
    pub digital_input_1: bool,
    pub digital_input_2: bool,
    pub digital_input_3: bool,
    pub digital_output_1: bool,
    pub digital_output_2: bool,
    pub digital_output_3: bool,
    pub robot_speed: u8,
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
                        joint_1: buffer[1],
                        joint_2: buffer[2],
                        joint_3: buffer[3],
                        joint_4: buffer[4],
                        joint_5: buffer[5],
                        joint_6: buffer[6],
                        digital_input_1: buffer[7] != 0,
                        digital_input_2: buffer[8] != 0,
                        digital_input_3: buffer[9] != 0,
                        digital_output_1: buffer[10] != 0,
                        digital_output_2: buffer[11] != 0,
                        digital_output_3: buffer[12] != 0,
                        robot_speed: buffer[13],
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
    data[1] = robot_state.joint_1;
    data[2] = robot_state.joint_2;
    data[3] = robot_state.joint_3;
    data[4] = robot_state.joint_4;
    data[5] = robot_state.joint_5;
    data[6] = robot_state.joint_6;
    data[7] = robot_state.digital_input_1 as u8;
    data[8] = robot_state.digital_input_2 as u8;
    data[9] = robot_state.digital_input_3 as u8;
    data[10] = robot_state.digital_output_1 as u8;
    data[11] = robot_state.digital_output_2 as u8;
    data[12] = robot_state.digital_output_3 as u8;
    data[13] = robot_state.robot_speed;
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
