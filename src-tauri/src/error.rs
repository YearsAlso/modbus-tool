//! Error types for Modbus Tool

use thiserror::Error;

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Main error enum for the application
#[derive(Error, Debug)]
pub enum Error {
    // ========== Serial Port Errors ==========
    #[error("Serial port error: {0}")]
    SerialPort(String),

    #[error("Failed to list serial ports: {0}")]
    SerialPortList(String),

    #[error("Failed to open serial port: {0}")]
    SerialPortOpen(String),

    #[error("Serial port not open")]
    SerialPortNotOpen,

    #[error("Serial port already open: {0}")]
    SerialPortAlreadyOpen(String),

    // ========== TCP/IP Errors ==========
    #[error("TCP connection error: {0}")]
    TcpConnection(String),

    #[error("TCP connection not found: {0}")]
    TcpConnectionNotFound(String),

    #[error("TCP connection already exists: {0}")]
    TcpConnectionAlreadyExists(String),

    #[error("TCP read error: {0}")]
    TcpRead(String),

    #[error("TCP write error: {0}")]
    TcpWrite(String),

    // ========== Modbus Protocol Errors ==========
    #[error("Modbus error: {0}")]
    Modbus(String),

    #[error("Invalid Modbus response: {0}")]
    InvalidModbusResponse(String),

    #[error("Modbus CRC error: expected {expected:#x}, got {got:#x}")]
    ModbusCrcError { expected: u16, got: u16 },

    #[error("Modbus timeout")]
    ModbusTimeout,

    #[error("Modbus invalid function code: {0}")]
    ModbusInvalidFunction(u8),

    // ========== IO Errors ==========
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    // ========== Configuration Errors ==========
    #[error("Invalid configuration: {0}")]
    Config(String),

    // ========== General Errors ==========
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Not connected")]
    NotConnected,

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Get error code for UI display
    pub fn code(&self) -> i32 {
        match self {
            // Serial port errors: 1000-1999
            Error::SerialPort(_) => 1000,
            Error::SerialPortList(_) => 1001,
            Error::SerialPortOpen(_) => 1002,
            Error::SerialPortNotOpen => 1003,
            Error::SerialPortAlreadyOpen(_) => 1004,

            // TCP errors: 2000-2999
            Error::TcpConnection(_) => 2000,
            Error::TcpConnectionNotFound(_) => 2001,
            Error::TcpConnectionAlreadyExists(_) => 2002,
            Error::TcpRead(_) => 2003,
            Error::TcpWrite(_) => 2004,

            // Modbus errors: 3000-3999
            Error::Modbus(_) => 3000,
            Error::InvalidModbusResponse(_) => 3001,
            Error::ModbusCrcError { .. } => 3002,
            Error::ModbusTimeout => 3003,
            Error::ModbusInvalidFunction(_) => 3004,

            // IO/Config errors: 4000-4999
            Error::Io(_) => 4000,
            Error::Parse(_) => 4001,
            Error::Config(_) => 4002,

            // General errors: 5000-5999
            Error::Connection(_) => 5000,
            Error::NotConnected => 5001,
            Error::Cancelled => 5002,
            Error::Timeout(_) => 5003,
            Error::Other(_) => 5100,
        }
    }
}

impl From<serialport::Error> for Error {
    fn from(e: serialport::Error) -> Self {
        Error::SerialPort(e.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::Parse(e.to_string())
    }
}
