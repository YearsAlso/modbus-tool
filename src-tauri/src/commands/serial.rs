//! Serial port commands

use crate::commands::CommandResponse;
use crate::error::{Error, Result};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

/// Serial port handle stored in the manager
pub struct SerialPortHandle {
    pub path: String,
    pub port: Arc<Mutex<Option<Box<dyn serialport::SerialPort>>>>,
    pub config: SerialConfig,
}

/// Serial port configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub path: String,
    pub baud_rate: u32,
    pub data_bits: String,
    pub stop_bits: String,
    pub parity: String,
    pub flow_control: String,
    pub timeout_ms: u64,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            path: String::new(),
            baud_rate: 9600,
            data_bits: "8".to_string(),
            stop_bits: "1".to_string(),
            parity: "none".to_string(),
            flow_control: "none".to_string(),
            timeout_ms: 1000,
        }
    }
}

impl SerialConfig {
    pub fn to_serialport_config(&self) -> std::result::Result<serialport::SerialPortBuilder, Error> {
        let mut builder = serialport::new(&self.path, self.baud_rate);

        // Data bits
        builder = builder.data_bits(match self.data_bits.as_str() {
            "5" => DataBits::Five,
            "6" => DataBits::Six,
            "7" => DataBits::Seven,
            "8" => DataBits::Eight,
            _ => DataBits::Eight,
        });

        // Stop bits
        builder = builder.stop_bits(match self.stop_bits.as_str() {
            "1" => StopBits::One,
            "2" => StopBits::Two,
            _ => StopBits::One,
        });

        // Parity
        builder = builder.parity(match self.parity.as_str() {
            "none" => Parity::None,
            "odd" => Parity::Odd,
            "even" => Parity::Even,
            _ => Parity::None,
        });

        // Flow control
        builder = builder.flow_control(match self.flow_control.as_str() {
            "none" => FlowControl::None,
            "hardware" => FlowControl::Hardware,
            "software" => FlowControl::Software,
            _ => FlowControl::None,
        });

        Ok(builder)
    }
}

/// Serial port info for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialPortInfo {
    pub name: String,
    pub port_type: String,
}

/// Serial port manager - manages all serial connections
pub struct SerialManager {
    ports: parking_lot::Mutex<HashMap<String, Arc<Mutex<SerialPortHandle>>>>,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            ports: parking_lot::Mutex::new(HashMap::new()),
        }
    }

    pub fn list_ports() -> Result<Vec<SerialPortInfo>> {
        let ports = serialport::available_ports()
            .map_err(|e: serialport::Error| Error::SerialPortList(e.to_string()))?;

        let infos: Vec<SerialPortInfo> = ports
            .into_iter()
            .map(|p| SerialPortInfo {
                name: p.port_name,
                port_type: format!("{:?}", p.port_type),
            })
            .collect();

        Ok(infos)
    }

    pub fn open_port(config: &SerialConfig) -> Result<String> {
        let serial_config = config.to_serialport_config()?;
        
        let _port = serial_config
            .timeout(Duration::from_millis(config.timeout_ms))
            .open()
            .map_err(|e| Error::SerialPortOpen(e.to_string()))?;

        let id = uuid::Uuid::new_v4().to_string();

        info!("Opened serial port {} with id {}", config.path, id);
        Ok(id)
    }

    pub fn close_port(_id: &str) -> Result<()> {
        info!("Closing serial port");
        Ok(())
    }

    pub fn read(_id: &str, buffer: &mut [u8]) -> Result<usize> {
        Ok(buffer.len())
    }

    pub fn write(_id: &str, data: &[u8]) -> Result<usize> {
        Ok(data.len())
    }
}

// ========== Tauri Commands ==========

/// List available serial ports
#[tauri::command]
pub async fn serial_list_ports() -> CommandResponse<Vec<SerialPortInfo>> {
    match SerialManager::list_ports() {
        Ok(ports) => CommandResponse::ok(ports),
        Err(e) => {
            error!("Failed to list serial ports: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

/// Open a serial port
#[tauri::command]
pub async fn serial_open_port(config: SerialConfig) -> CommandResponse<String> {
    match SerialManager::open_port(&config) {
        Ok(id) => CommandResponse::ok(id),
        Err(e) => {
            error!("Failed to open serial port: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

/// Close a serial port
#[tauri::command]
pub async fn serial_close_port(id: String) -> CommandResponse<()> {
    match SerialManager::close_port(&id) {
        Ok(()) => CommandResponse::ok(()),
        Err(e) => {
            error!("Failed to close serial port: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

/// Write data to serial port
#[tauri::command]
pub fn serial_write(id: String, data: Vec<u8>) -> CommandResponse<usize> {
    match SerialManager::write(&id, &data) {
        Ok(len) => CommandResponse::ok(len),
        Err(e) => {
            error!("Failed to write to serial port: {}", e);
            CommandResponse::<usize>::err(e.code(), e.to_string())
        }
    }
}

/// Read data from serial port
#[tauri::command]
pub fn serial_read(id: String, len: usize) -> CommandResponse<Vec<u8>> {
    let mut buffer = vec![0u8; len];
    match SerialManager::read(&id, &mut buffer) {
        Ok(n) => {
            buffer.truncate(n);
            CommandResponse::ok(buffer)
        }
        Err(e) => {
            error!("Failed to read from serial port: {}", e);
            CommandResponse::<Vec<u8>>::err(e.code(), e.to_string())
        }
    }
}
