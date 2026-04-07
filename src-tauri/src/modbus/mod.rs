//! Modbus Protocol Implementation Module
//!
//! Contains protocol parsing, encoding/decoding for RTU and TCP modes

pub mod protocol;
pub mod rtu;
pub mod tcp;

// Re-export commonly used items
pub use protocol::*;

// Modbus communication mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModbusMode {
    /// Modbus RTU over serial (RS-232/RS-485)
    Rtu,
    /// Modbus TCP over Ethernet
    Tcp,
    /// Modbus ASCII over serial
    Ascii,
    /// Modbus UDP over Ethernet
    Udp,
}

impl Default for ModbusMode {
    fn default() -> Self {
        Self::Rtu
    }
}

impl std::fmt::Display for ModbusMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModbusMode::Rtu => write!(f, "RTU"),
            ModbusMode::Tcp => write!(f, "TCP"),
            ModbusMode::Ascii => write!(f, "ASCII"),
            ModbusMode::Udp => write!(f, "UDP"),
        }
    }
}
