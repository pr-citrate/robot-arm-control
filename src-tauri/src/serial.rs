// src-tauri/serial.rs

use serde::{Deserialize, Serialize};
use serialport;
use std::io::{ErrorKind, Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::State;

// RobotState 구조체 정의
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

// SerialPortManager 구조체 정의
pub struct SerialPortManager {
    port: Arc<Mutex<Option<Box<dyn serialport::SerialPort + Send>>>>,
}

impl SerialPortManager {
    pub fn new() -> Self {
        Self {
            port: Arc::new(Mutex::new(None)),
        }
    }

    // 시리얼 포트 초기화 함수
    pub fn initialize(&self, port_name: &str, baud_rate: u32) -> Result<(), serialport::Error> {
        let s = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()?;
        let mut port_lock = self.port.lock().unwrap();
        *port_lock = Some(s);
        Ok(())
    }

    // 데이터 전송 함수
    pub fn send_data(&self, data: &[u8]) -> Result<(), serialport::Error> {
        let mut port_lock = self.port.lock().unwrap();
        if let Some(ref mut port) = *port_lock {
            port.write_all(data)?;
            // 데이터 전송 로그
            println!("Sent data: {:?}", data);
            Ok(())
        } else {
            Err(serialport::Error::new(
                serialport::ErrorKind::Io(ErrorKind::Other),
                "Serial port not initialized",
            ))
        }
    }

    // 데이터 수신 함수
    pub fn read_data(&self) -> Result<RobotState, String> {
        let mut port_lock = self.port.lock().unwrap();
        if let Some(ref mut port) = *port_lock {
            let mut buffer: Vec<u8> = Vec::new();
            let mut byte: u8;

            // 헤드 바이트(253) 찾기
            loop {
                let mut single_byte = [0u8; 1];
                match port.read_exact(&mut single_byte) {
                    Ok(_) => {
                        byte = single_byte[0];
                        if byte == 253 {
                            buffer.push(byte);
                            break;
                        }
                    },
                    Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                        return Err("데이터를 기다리는 동안 타임아웃이 발생했습니다.".into());
                    },
                    Err(e) => {
                        return Err(format!("시리얼 포트 읽기 오류: {}", e));
                    },
                }
            }

            // 나머지 14바이트 읽기
            let mut remaining_bytes = [0u8; 14];
            match port.read_exact(&mut remaining_bytes) {
                Ok(_) => {
                    buffer.extend_from_slice(&remaining_bytes);
                    // 수신 데이터 로그
                    println!("Received data: {:?}", buffer);

                    if buffer.len() != 15 || buffer[14] != 254 {
                        return Err("유효하지 않은 데이터 패킷: 잘못된 테일 바이트".into());
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
                },
                Err(e) => {
                    return Err(format!("나머지 데이터 읽기 오류: {}", e));
                },
            }
        } else {
            Err("시리얼 포트가 초기화되지 않았습니다.".into())
        }
    }

    // 시리얼 포트 목록 가져오기 함수
    pub fn list_ports() -> Result<Vec<serialport::SerialPortInfo>, serialport::Error> {
        serialport::available_ports()
    }
}

// AppState 구조체 정의
#[derive(Clone)]
pub struct AppState {
    pub serial_manager: Arc<SerialPortManager>,
}

// 시리얼 포트 목록 커맨드
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
        Err(e) => Err(format!("시리얼 포트 목록 가져오기 실패: {}", e)),
    }
}

// 시리얼 포트 초기화 커맨드
#[tauri::command]
pub fn initialize_serial(
    state: State<'_, AppState>,
    port: String,
    baud_rate: u32,
) -> Result<String, String> {
    match state.serial_manager.initialize(&port, baud_rate) {
        Ok(_) => Ok("시리얼 포트가 성공적으로 초기화되었습니다.".into()),
        Err(e) => Err(format!("시리얼 포트 열기 실패: {}", e)),
    }
}

// 로봇 명령 전송 커맨드
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

    // 데이터 전송 로그
    println!("Sending robot commands: {:?}", data);

    state
        .serial_manager
        .send_data(&data)
        .map_err(|e| format!("데이터 전송 실패: {}", e))?;

    Ok(())
}

// 로봇 상태 읽기 커맨드
#[tauri::command]
pub fn read_robot_state(state: State<'_, AppState>) -> Result<RobotState, String> {
    match state.serial_manager.read_data() {
        Ok(robot_state) => Ok(robot_state),
        Err(e) => Err(format!("로봇 상태 읽기 실패: {}", e)),
    }
}
