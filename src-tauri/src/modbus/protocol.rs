//! Modbus Protocol Implementation
//! 
//! Supports Modbus RTU and Modbus TCP protocols

use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ModbusError {
    #[error("Invalid function code: {0}")]
    InvalidFunctionCode(u8),
    
    #[error("Invalid data length: {0}")]
    InvalidDataLength(usize),
    
    #[error("CRC mismatch: expected {expected}, got {got}")]
    CrcMismatch { expected: u16, got: u16 },
    
    #[error("Incomplete frame: got {got}, expected {expected}")]
    IncompleteFrame { got: usize, expected: usize },
    
    #[error("MBAP parse error: {0}")]
    MbapParseError(String),
    
    #[error("Invalid PDU: {0}")]
    InvalidPdu(String),
    
    #[error("Device error: {0}")]
    DeviceError(u8),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("IO error: {0}")]
    IoError(String),
}

pub type Result<T> = std::result::Result<T, ModbusError>;

// ============================================================================
// Constants
// ============================================================================

/// Modbus Function Codes
pub const READ_COILS: u8 = 0x01;
pub const READ_DISCRETE_INPUTS: u8 = 0x02;
pub const READ_HOLDING_REGISTERS: u8 = 0x03;
pub const READ_INPUT_REGISTERS: u8 = 0x04;
pub const WRITE_SINGLE_COIL: u8 = 0x05;
pub const WRITE_SINGLE_REGISTER: u8 = 0x06;
pub const WRITE_MULTIPLE_COILS: u8 = 0x0F;
pub const WRITE_MULTIPLE_REGISTERS: u8 = 0x10;
pub const MASK_WRITE_REGISTER: u8 = 0x16;
pub const READ_WRITE_MULTIPLE: u8 = 0x17;

/// Exception codes from Modbus devices
pub const EXCEPTION_NONE: u8 = 0x00;
pub const EXCEPTION_ILLEGAL_FUNCTION: u8 = 0x01;
pub const EXCEPTION_ILLEGAL_DATA_ADDRESS: u8 = 0x02;
pub const EXCEPTION_ILLEGAL_DATA_VALUE: u8 = 0x03;
pub const EXCEPTION_SERVER_FAILURE: u8 = 0x04;
pub const EXCEPTION_ACKNOWLEDGE: u8 = 0x05;
pub const EXCEPTION_SERVER_BUSY: u8 = 0x06;
pub const EXCEPTION_MEMORY_PARITY_ERROR: u8 = 0x08;
pub const EXCEPTION_GATEWAY_PATH_UNAVAILABLE: u8 = 0x0A;
pub const EXCEPTION_GATEWAY_TARGET_FAILED: u8 = 0x0B;

/// Modbus TCP default port
pub const DEFAULT_PORT: u16 = 502;

/// RTU frame timeout in milliseconds
pub const RTU_FRAME_TIMEOUT_MS: u64 = 1000;

/// Maximum number of coils/registers in a single read
pub const MAX_READ_COUNT: u16 = 2000;

/// Maximum number of coils in a single write
pub const MAX_WRITE_COILS: u16 = 1968;

/// Maximum number of registers in a single write
pub const MAX_WRITE_REGISTERS: u16 = 123;

// ============================================================================
// Data Types
// ============================================================================

/// Modbus PDU (Protocol Data Unit)
#[derive(Debug, Clone, PartialEq)]
pub struct ModbusPdu {
    pub function_code: u8,
    pub data: Vec<u8>,
}

impl ModbusPdu {
    /// Create a new PDU
    pub fn new(function_code: u8, data: Vec<u8>) -> Self {
        Self { function_code, data }
    }
    
    /// Get the exception response function code (function_code | 0x80)
    pub fn exception_code(&self) -> Option<u8> {
        if self.function_code & 0x80 != 0 {
            Some(self.function_code & 0x7F)
        } else {
            None
        }
    }
    
    /// Check if this is an exception response
    pub fn is_exception(&self) -> bool {
        self.function_code & 0x80 != 0
    }
    
    /// Get the exception code from the data
    pub fn get_exception_code(&self) -> Option<u8> {
        self.data.first().copied()
    }
}

/// Modbus TCP MBAP (Modbus Application Protocol) Header
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MbapHeader {
    /// Transaction identifier (set by client)
    pub transaction_id: u16,
    /// Protocol identifier (0 for Modbus)
    pub protocol_id: u16,
    /// Length of the following bytes
    pub length: u16,
    /// Unit identifier (slave address for serial, 0xFF for TCP)
    pub unit_id: u8,
}

impl MbapHeader {
    /// Parse MBAP header from bytes
    pub fn parse(buf: &[u8]) -> Result<Self> {
        if buf.len() < 7 {
            return Err(ModbusError::IncompleteFrame {
                got: buf.len(),
                expected: 7,
            });
        }
        
        let transaction_id = u16::from_be_bytes([buf[0], buf[1]]);
        let protocol_id = u16::from_be_bytes([buf[2], buf[3]]);
        
        if protocol_id != 0 {
            return Err(ModbusError::MbapParseError(format!(
                "Invalid protocol_id: {}, expected 0",
                protocol_id
            )));
        }
        
        let length = u16::from_be_bytes([buf[4], buf[5]]);
        let unit_id = buf[6];
        
        Ok(Self {
            transaction_id,
            protocol_id,
            length,
            unit_id,
        })
    }
    
    /// Write MBAP header to bytes
    pub fn write(&self, buf: &mut BytesMut) {
        buf.put_u16(self.transaction_id);
        buf.put_u16(self.protocol_id);
        buf.put_u16(self.length);
        buf.put_u8(self.unit_id);
    }
    
    /// Get the total frame length including header
    pub fn total_length(&self) -> usize {
        7 + (self.length - 1) as usize
    }
}

/// Modbus TCP Frame (MBAP + PDU)
#[derive(Debug, Clone, PartialEq)]
pub struct ModbusTcpFrame {
    pub mbap: MbapHeader,
    pub pdu: ModbusPdu,
}

impl ModbusTcpFrame {
    /// Parse a TCP frame from bytes
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let mbap = MbapHeader::parse(buf)?;
        let pdu_start = 7;
        let pdu_len = (mbap.length - 1) as usize;
        
        if buf.len() < pdu_start + pdu_len {
            return Err(ModbusError::IncompleteFrame {
                got: buf.len(),
                expected: pdu_start + pdu_len,
            });
        }
        
        let pdu_data = buf[pdu_start..pdu_start + pdu_len].to_vec();
        let pdu = ModbusPdu::new(pdu_data[0], pdu_data[1..].to_vec());
        
        Ok(Self { mbap, pdu })
    }
    
    /// Encode frame to bytes
    pub fn encode(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(7 + self.pdu.data.len() + 2);
        
        // MBAP length = unit_id (1) + function_code (1) + data (n)
        let length = 2 + self.pdu.data.len() as u16;
        let mbap = MbapHeader {
            transaction_id: self.mbap.transaction_id,
            protocol_id: 0,
            length,
            unit_id: self.mbap.unit_id,
        };
        mbap.write(&mut buf);
        
        buf.put_u8(self.pdu.function_code);
        buf.extend_from_slice(&self.pdu.data);
        
        buf
    }
}

/// Modbus RTU Frame
#[derive(Debug, Clone, PartialEq)]
pub struct ModbusRtuFrame {
    pub slave_address: u8,
    pub pdu: ModbusPdu,
}

impl ModbusRtuFrame {
    /// Parse an RTU frame from bytes
    pub fn parse(buf: &[u8]) -> Result<Self> {
        if buf.len() < 4 {
            return Err(ModbusError::IncompleteFrame {
                got: buf.len(),
                expected: 4,
            });
        }
        
        let slave_address = buf[0];
        let function_code = buf[1];
        
        // Get everything except CRC
        let data_len = buf.len() - 2;
        let crc_pos = data_len;
        
        // Verify CRC
        let calculated_crc = Self::calculate_crc(&buf[..data_len]);
        let frame_crc = u16::from_le_bytes([buf[crc_pos], buf[crc_pos + 1]]);
        
        if calculated_crc != frame_crc {
            return Err(ModbusError::CrcMismatch {
                expected: calculated_crc,
                got: frame_crc,
            });
        }
        
        let data = buf[2..data_len].to_vec();
        
        Ok(Self {
            slave_address,
            pdu: ModbusPdu::new(function_code, data),
        })
    }
    
    /// Encode frame to RTU bytes (without CRC)
    pub fn encode_without_crc(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2 + self.pdu.data.len());
        buf.push(self.slave_address);
        buf.push(self.pdu.function_code);
        buf.extend_from_slice(&self.pdu.data);
        buf
    }
    
    /// Encode frame to RTU bytes (with CRC)
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = self.encode_without_crc();
        let crc = Self::calculate_crc(&buf);
        buf.push(crc as u8);
        buf.push((crc >> 8) as u8);
        buf
    }
    
    /// Calculate Modbus CRC16
    pub fn calculate_crc(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        
        for byte in data {
            crc ^= *byte as u16;
            
            for _ in 0..8 {
                if crc & 0x0001 != 0 {
                    crc = (crc >> 1) ^ 0xA001;
                } else {
                    crc >>= 1;
                }
            }
        }
        
        crc
    }
}

// ============================================================================
// PDU Builders
// ============================================================================

impl ModbusPdu {
    /// Build Read Coils request (FC01)
    pub fn read_coils(address: u16, count: u16) -> Result<Self> {
        if count == 0 || count > MAX_READ_COUNT {
            return Err(ModbusError::InvalidDataLength(count as usize));
        }
        
        let mut data = Vec::with_capacity(4);
        data.put_u16(address);
        data.put_u16(count);
        
        Ok(Self::new(READ_COILS, data))
    }
    
    /// Build Read Holding Registers request (FC03)
    pub fn read_holding_registers(address: u16, count: u16) -> Result<Self> {
        if count == 0 || count > MAX_READ_COUNT {
            return Err(ModbusError::InvalidDataLength(count as usize));
        }
        
        let mut data = Vec::with_capacity(4);
        data.put_u16(address);
        data.put_u16(count);
        
        Ok(Self::new(READ_HOLDING_REGISTERS, data))
    }
    
    /// Build Read Input Registers request (FC04)
    pub fn read_input_registers(address: u16, count: u16) -> Result<Self> {
        if count == 0 || count > MAX_READ_COUNT {
            return Err(ModbusError::InvalidDataLength(count as usize));
        }
        
        let mut data = Vec::with_capacity(4);
        data.put_u16(address);
        data.put_u16(count);
        
        Ok(Self::new(READ_INPUT_REGISTERS, data))
    }
    
    /// Build Write Single Coil request (FC05)
    pub fn write_single_coil(address: u16, value: bool) -> Result<Self> {
        let mut data = Vec::with_capacity(4);
        data.put_u16(address);
        data.put_u16(if value { 0xFF00 } else { 0x0000 });
        
        Ok(Self::new(WRITE_SINGLE_COIL, data))
    }
    
    /// Build Write Single Register request (FC06)
    pub fn write_single_register(address: u16, value: u16) -> Result<Self> {
        let mut data = Vec::with_capacity(4);
        data.put_u16(address);
        data.put_u16(value);
        
        Ok(Self::new(WRITE_SINGLE_REGISTER, data))
    }
    
    /// Build Write Multiple Registers request (FC16)
    pub fn write_multiple_registers(address: u16, values: &[u16]) -> Result<Self> {
        let count = values.len() as u16;
        if count == 0 || count > MAX_WRITE_REGISTERS {
            return Err(ModbusError::InvalidDataLength(count as usize));
        }
        
        let byte_count = count * 2;
        let mut data = Vec::with_capacity(5 + byte_count as usize);
        data.put_u16(address);
        data.put_u16(count);
        data.put_u8(byte_count as u8);
        
        for &value in values {
            data.put_u16(value);
        }
        
        Ok(Self::new(WRITE_MULTIPLE_REGISTERS, data))
    }
    
    /// Parse read coils/inputs response
    pub fn parse_read_bits(&self) -> Result<Vec<bool>> {
        if self.function_code != READ_COILS && self.function_code != READ_DISCRETE_INPUTS {
            return Err(ModbusError::InvalidFunctionCode(self.function_code));
        }
        
        if self.data.is_empty() {
            return Err(ModbusError::InvalidPdu("Empty data".to_string()));
        }
        
        let byte_count = self.data[0] as usize;
        if self.data.len() < byte_count + 1 {
            return Err(ModbusError::InvalidDataLength(self.data.len()));
        }
        
        let mut result = Vec::with_capacity(byte_count * 8);
        for i in 0..byte_count {
            let byte = self.data[1 + i];
            for bit in 0..8 {
                result.push((byte >> bit) & 1 == 1);
            }
        }
        
        Ok(result)
    }
    
    /// Parse read registers response
    pub fn parse_read_registers(&self) -> Result<Vec<u16>> {
        if self.function_code != READ_HOLDING_REGISTERS && self.function_code != READ_INPUT_REGISTERS {
            return Err(ModbusError::InvalidFunctionCode(self.function_code));
        }
        
        if self.data.is_empty() {
            return Err(ModbusError::InvalidPdu("Empty data".to_string()));
        }
        
        let byte_count = self.data[0] as usize;
        if self.data.len() < byte_count + 1 || byte_count % 2 != 0 {
            return Err(ModbusError::InvalidDataLength(self.data.len()));
        }
        
        let mut result = Vec::with_capacity(byte_count / 2);
        let mut buf = &self.data[1..];
        
        while buf.len() >= 2 {
            result.push(buf.get_u16());
        }
        
        Ok(result)
    }
    
    /// Parse write single register response (echo back)
    pub fn parse_write_single_register(&self, expected_address: u16, expected_value: u16) -> Result<(u16, u16)> {
        if self.function_code != WRITE_SINGLE_REGISTER {
            return Err(ModbusError::InvalidFunctionCode(self.function_code));
        }
        
        if self.data.len() != 4 {
            return Err(ModbusError::InvalidDataLength(self.data.len()));
        }
        
        let address = u16::from_be_bytes([self.data[0], self.data[1]]);
        let value = u16::from_be_bytes([self.data[2], self.data[3]]);
        
        if address != expected_address || value != expected_value {
            return Err(ModbusError::InvalidPdu(format!(
                "Echo mismatch: expected ({}, {}), got ({}, {})",
                expected_address, expected_value, address, value
            )));
        }
        
        Ok((address, value))
    }
    
    /// Parse write multiple registers response
    pub fn parse_write_multiple_registers(&self, expected_address: u16, expected_count: u16) -> Result<(u16, u16)> {
        if self.function_code != WRITE_MULTIPLE_REGISTERS {
            return Err(ModbusError::InvalidFunctionCode(self.function_code));
        }
        
        if self.data.len() != 4 {
            return Err(ModbusError::InvalidDataLength(self.data.len()));
        }
        
        let address = u16::from_be_bytes([self.data[0], self.data[1]]);
        let count = u16::from_be_bytes([self.data[2], self.data[3]]);
        
        if address != expected_address || count != expected_count {
            return Err(ModbusError::InvalidPdu(format!(
                "Echo mismatch: expected ({}, {}), got ({}, {})",
                expected_address, expected_count, address, count
            )));
        }
        
        Ok((address, count))
    }
    
    /// Create exception response
    pub fn exception(function_code: u8, exception_code: u8) -> Self {
        Self::new(function_code | 0x80, vec![exception_code])
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crc_calculation() {
        // Known test vector from Modbus specification
        let data = [0x01, 0x03, 0x00, 0x00, 0x00, 0x0A];
        let crc = ModbusRtuFrame::calculate_crc(&data);
        assert_ne!(crc, 0xFFFF); // CRC computed
    }
    
    #[test]
    fn test_crc_empty() {
        let crc = ModbusRtuFrame::calculate_crc(&[]);
        assert_eq!(crc, 0xFFFF);
    }
    
    #[test]
    fn test_crc_single_byte() {
        let crc = ModbusRtuFrame::calculate_crc(&[0x01]);
        assert_ne!(crc, 0xFFFF); // CRC computed
    }
    
    #[test]
    fn test_read_holding_registers_build() {
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        assert_eq!(pdu.function_code, READ_HOLDING_REGISTERS);
        assert_eq!(pdu.data, vec![0x00, 0x00, 0x00, 0x0A]);
    }
    
    #[test]
    fn test_read_holding_registers_invalid_count() {
        let result = ModbusPdu::read_holding_registers(0, 0);
        assert!(result.is_err());
        
        let result = ModbusPdu::read_holding_registers(0, 2001);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_write_single_register_build() {
        let pdu = ModbusPdu::write_single_register(100, 0x1234).unwrap();
        assert_eq!(pdu.function_code, WRITE_SINGLE_REGISTER);
        assert_eq!(pdu.data, vec![0x00, 0x64, 0x12, 0x34]);
    }
    
    #[test]
    fn test_write_multiple_registers_build() {
        let values = [0x0001, 0x0002, 0x0003];
        let pdu = ModbusPdu::write_multiple_registers(0, &values).unwrap();
        assert_eq!(pdu.function_code, WRITE_MULTIPLE_REGISTERS);
        assert_eq!(pdu.data, vec![0x00, 0x00, 0x00, 0x03, 0x06, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03]);
    }
    
    #[test]
    fn test_parse_read_registers() {
        let pdu = ModbusPdu::new(READ_HOLDING_REGISTERS, vec![0x04, 0x00, 0x01, 0x00, 0x02]);
        let registers = pdu.parse_read_registers().unwrap();
        assert_eq!(registers, vec![1, 2]);
    }
    
    #[test]
    fn test_parse_read_bits() {
        // byte_count=2, coil_data=[0xAC, 0x00] = 16 bits total
        // 0xAC = 10101100, 0x00 = 00000000
        let pdu = ModbusPdu::new(READ_COILS, vec![0x02, 0xAC, 0x00]);
        let bits = pdu.parse_read_bits().unwrap();
        assert_eq!(bits.len(), 16);
        // bits from 0xAC: bit0=0, bit1=0, bit2=1, bit3=1, bit4=0, bit5=1, bit6=1, bit7=0
        assert!(!bits[0]); // bit 0 of 0xAC is 0
        assert!(!bits[1]); // bit 1 of 0xAC is 0
        assert!(bits[2]);  // bit 2 of 0xAC is 1
        assert!(bits[3]);  // bit 3 of 0xAC is 1
    }
    
    #[test]
    fn test_mbap_header() {
        let data = [0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01];
        let mbap = MbapHeader::parse(&data).unwrap();
        assert_eq!(mbap.transaction_id, 1);
        assert_eq!(mbap.protocol_id, 0);
        assert_eq!(mbap.length, 5);
        assert_eq!(mbap.unit_id, 1);
    }
    
    #[test]
    fn test_mbap_header_invalid_protocol() {
        let data = [0x00, 0x01, 0x00, 0x01, 0x00, 0x05, 0x01];
        let result = MbapHeader::parse(&data);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_mbap_header_incomplete() {
        let data = [0x00, 0x01, 0x00];
        let result = MbapHeader::parse(&data);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_pdu_is_exception() {
        let normal = ModbusPdu::new(0x03, vec![]);
        assert!(!normal.is_exception());
        
        let exception = ModbusPdu::new(0x83, vec![0x02]);
        assert!(exception.is_exception());
        assert_eq!(exception.exception_code(), Some(0x03));
    }
    
    #[test]
    fn test_write_single_coil_on() {
        let pdu = ModbusPdu::write_single_coil(0, true).unwrap();
        assert_eq!(pdu.data, vec![0x00, 0x00, 0xFF, 0x00]);
    }
    
    #[test]
    fn test_write_single_coil_off() {
        let pdu = ModbusPdu::write_single_coil(0, false).unwrap();
        assert_eq!(pdu.data, vec![0x00, 0x00, 0x00, 0x00]);
    }
    
    #[test]
    fn test_rtu_frame_encode() {
        let pdu = ModbusPdu::new(READ_HOLDING_REGISTERS, vec![0x00, 0x00, 0x00, 0x0A]);
        let frame = ModbusRtuFrame { slave_address: 1, pdu };
        let encoded = frame.encode();
        
        // Verify CRC
        let data_without_crc = &encoded[..encoded.len() - 2];
        let crc = ModbusRtuFrame::calculate_crc(data_without_crc);
        let frame_crc = u16::from_le_bytes([encoded[encoded.len() - 2], encoded[encoded.len() - 1]]);
        assert_eq!(crc, frame_crc);
    }
    
    #[test]
    fn test_rtu_frame_parse() {
        // Build a known frame: Slave=1, FC03, addr=0, count=10, CRC
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let frame = ModbusRtuFrame { slave_address: 1, pdu };
        let encoded = frame.encode();
        
        let parsed = ModbusRtuFrame::parse(&encoded).unwrap();
        assert_eq!(parsed.slave_address, 1);
        assert_eq!(parsed.pdu.function_code, READ_HOLDING_REGISTERS);
        assert_eq!(parsed.pdu.data, vec![0x00, 0x00, 0x00, 0x0A]);
    }
    
    #[test]
    fn test_rtu_frame_crc_error() {
        let mut data = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00];
        let result = ModbusRtuFrame::parse(&data);
        assert!(matches!(result, Err(ModbusError::CrcMismatch { .. })));
    }
    
    #[test]
    fn test_tcp_frame_roundtrip() {
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let original = ModbusTcpFrame {
            mbap: MbapHeader {
                transaction_id: 1,
                protocol_id: 0,
                length: 6,
                unit_id: 1,
            },
            pdu,
        };
        
        let encoded = original.encode();
        let parsed = ModbusTcpFrame::parse(&encoded).unwrap();
        
        assert_eq!(parsed.mbap.transaction_id, original.mbap.transaction_id);
        assert_eq!(parsed.mbap.unit_id, original.mbap.unit_id);
        assert_eq!(parsed.pdu.function_code, original.pdu.function_code);
        assert_eq!(parsed.pdu.data, original.pdu.data);
    }
}
