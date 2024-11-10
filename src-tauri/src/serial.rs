use serialport;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::sync::{Arc, Mutex};
//use std::thread;
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
    port: Arc<Mutex<Box<dyn serialport::SerialPort>>>,
}

impl SerialPortManager {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, serialport::Error> {
        let s = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()?;
        Ok(Self {
            port: Arc::new(Mutex::new(s)),
        })
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), serialport::Error> {
        let mut port = self.port.lock().unwrap();
        port.write_all(data)?;
        Ok(())
    }

    pub fn read_data(&self) -> Result<Vec<u8>, serialport::Error> {
        let mut port = self.port.lock().unwrap();
        let mut buffer: Vec<u8> = vec![0; 15];
        match port.read_exact(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub serial_manager: Arc<SerialPortManager>,
}

#[tauri::command]
pub fn initialize_serial(port: String, baud_rate: u32) -> Result<String, String> {
    match SerialPortManager::new(&port, baud_rate) {
        Ok(manager) => {
            let _state = AppState {
                serial_manager: Arc::new(manager),
            };
            // Here you might want to store the state globally
            // For simplicity, we'll skip it in this example
            Ok("Serial port initialized".into())
        }
        Err(e) => Err(format!("Failed to open serial port: {}", e)),
    }
}

#[tauri::command]
pub fn send_robot_commands(state: State<'_, AppState>, robot_state: RobotState) -> Result<(), String> {
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
    let data = state
        .serial_manager
        .read_data()
        .map_err(|e| format!("Failed to read data: {}", e))?;

    if data.len() != 15 || data[0] != 253 || data[14] != 254 {
        return Err("Invalid data packet".into());
    }

    Ok(RobotState {
        J1: data[1],
        J2: data[2],
        J3: data[3],
        J4: data[4],
        J5: data[5],
        J6: data[6],
        Di1: data[7] != 0,
        Di2: data[8] != 0,
        Di3: data[9] != 0,
        Do1: data[10] != 0,
        Do2: data[11] != 0,
        Do3: data[12] != 0,
        robotSpeed: data[13],
    })
}
