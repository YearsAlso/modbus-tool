//! Modbus RTU Protocol Implementation
//!
//! Handles RTU frame assembly, inter-byte timeout detection, and CRC validation

use crate::modbus::protocol::{ModbusError, ModbusPdu, ModbusRtuFrame};
use std::time::{Duration, Instant};

/// Minimum inter-character timeout for Modbus RTU (1.5 character times)
/// At 9600bps 8N1: 1 char = 10 bits, so 1.5 chars = 15 bits = 1.56ms
pub const RTU_INTER_CHAR_TIMEOUT_MS: u64 = 2;

/// Frame timeout: 3.5 character times of silence marks end of frame
/// At 9600bps 8N1: 3.5 chars = 35 bits = 3.65ms; we use 4ms as safe margin
pub const RTU_FRAME_TIMEOUT_MS: u64 = 4;

/// Minimum frame size: slave addr (1) + function code (1) + CRC (2) = 4 bytes
pub const MIN_RTU_FRAME_SIZE: usize = 4;

/// RTU Frame Assembler
///
/// Accumulates bytes from a serial stream and emits complete RTU frames
/// when inter-character timeout is detected.
#[derive(Debug)]
pub struct RtuFramer {
    /// Accumulated bytes for current frame
    buffer: Vec<u8>,
    /// Timestamp of last received byte
    last_byte_time: Option<Instant>,
    /// Baud rate for character time calculation
    baud_rate: u32,
    /// Expected minimum frame size
    min_frame_size: usize,
}

impl RtuFramer {
    /// Create a new RTU framer with default 9600 baud
    pub fn new() -> Self {
        Self::with_baud_rate(9600)
    }

    /// Create a new RTU framer with specified baud rate
    ///
    /// The baud rate is used to calculate accurate inter-character timeouts.
    /// At 8N1 (8 data bits, no parity, 1 stop bit = 10 bits per character):
    /// - char_time_ms = (10 * 1000) / baud_rate
    /// - inter_char_timeout = 1.5 * char_time_ms
    /// - frame_timeout = 3.5 * char_time_ms
    pub fn with_baud_rate(baud_rate: u32) -> Self {
        Self {
            buffer: Vec::with_capacity(260), // Max PDU (253) + addr(1) + CRC(2) + headroom
            last_byte_time: None,
            baud_rate,
            min_frame_size: MIN_RTU_FRAME_SIZE,
        }
    }

    fn char_time_ms(&self) -> u64 {
        // 10 bits per character (8 data + 1 start + 1 stop)
        // Use floor division: 10000/9600 = 1.04 -> 1ms
        let divisor = self.baud_rate.max(1) as u64;
        (10_000u64) / divisor
    }

    /// Calculate inter-character timeout based on baud rate
    ///
    /// Per Modbus spec: 1.5 character times between bytes in a frame.
    /// Uses ceiling division: ceil(char_time * 1.5)
    pub fn inter_char_timeout(&self) -> Duration {
        let char_time = self.char_time_ms();
        // Ceiling division: ((n * 3) + 1) / 2
        let timeout_ms = ((char_time * 3) + 1) / 2;
        Duration::from_millis(timeout_ms.max(1u64))
    }

    /// Calculate frame timeout based on baud rate
    ///
    /// Per Modbus spec: 3.5 character times of silence marks end of frame.
    /// Uses ceiling division: ceil(char_time * 3.5)
    pub fn frame_timeout(&self) -> Duration {
        let char_time = self.char_time_ms();
        // Ceiling division: ((n * 7) + 1) / 2
        let timeout_ms = ((char_time * 7) + 1) / 2;
        Duration::from_millis(timeout_ms.max(4u64))
    }

    /// Push a byte into the framer and check if a complete frame is ready
    ///
    /// Returns `Ok(Some(frame))` if a complete frame was assembled,
    /// `Ok(None)` if more bytes are needed,
    /// `Err` if there was a parse or validation error.
    pub fn push_byte(&mut self, byte: u8) -> Result<Option<ModbusRtuFrame>, ModbusError> {
        let now = Instant::now();

        // Check if inter-character timeout has elapsed
        // If so, the previous buffer was an incomplete frame (timeout clears it)
        if let Some(last_time) = self.last_byte_time {
            if now.duration_since(last_time) > self.inter_char_timeout() {
                // Timeout between characters - discard incomplete frame
                self.buffer.clear();
            }
        }

        self.buffer.push(byte);
        self.last_byte_time = Some(now);

        // Try to extract a frame if we have minimum bytes
        self.try_extract_frame()
    }

    /// Push multiple bytes into the framer
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<Vec<ModbusRtuFrame>, ModbusError> {
        let mut frames = Vec::new();
        for &byte in bytes {
            if let Some(frame) = self.push_byte(byte)? {
                frames.push(frame);
            }
        }
        Ok(frames)
    }

    /// Try to extract a complete frame from the buffer
    ///
    /// Returns `Some(frame)` if buffer contains a valid complete frame,
    /// `None` if more bytes are needed.
    fn try_extract_frame(&mut self) -> Result<Option<ModbusRtuFrame>, ModbusError> {
        if self.buffer.len() < self.min_frame_size {
            return Ok(None);
        }

        // Calculate expected frame length based on function code
        let expected_len = self.expected_frame_length()?;
        if self.buffer.len() < expected_len {
            return Ok(None);
        }

        // We have enough bytes for a complete frame
        let frame = ModbusRtuFrame::parse(&self.buffer[..expected_len])?;
        self.buffer.drain(..expected_len);
        self.last_byte_time = Some(Instant::now()); // Reset timer after complete frame
        Ok(Some(frame))
    }

    /// Calculate expected frame length based on function code in buffer
    fn expected_frame_length(&self) -> Result<usize, ModbusError> {
        if self.buffer.is_empty() {
            return Ok(self.min_frame_size);
        }

        let function_code = self.buffer[1];

        // Response length depends on function code
        match function_code {
            // Read functions: response has at least 1 byte (byte count)
            0x01 | 0x02 | 0x03 | 0x04 => {
                // Minimum: addr(1) + fc(1) + byte_count(1) + CRC(2) = 5
                // But we need to wait for the byte_count byte first
                if self.buffer.len() < 3 {
                    return Ok(self.min_frame_size);
                }
                let byte_count = self.buffer[2] as usize;
                Ok(3 + byte_count + 2) // addr + fc + byte_count + data + CRC
            }
            // Write single coil/register: fixed 6 bytes + CRC
            0x05 | 0x06 => Ok(6 + 2),
            // Write multiple coils: addr(1) + fc(1) + start(2) + count(2) + byte_count(1) + CRC(2)
            0x0F => {
                if self.buffer.len() < 7 {
                    return Ok(self.min_frame_size);
                }
                let byte_count = self.buffer[6] as usize;
                Ok(7 + byte_count + 2)
            }
            // Write multiple registers
            0x10 => {
                if self.buffer.len() < 7 {
                    return Ok(self.min_frame_size);
                }
                let byte_count = self.buffer[6] as usize;
                Ok(7 + byte_count + 2)
            }
            // Exception response (function_code | 0x80)
            fc if fc & 0x80 != 0 => Ok(3 + 2), // addr + fc + exception + CRC
            _ => Ok(self.min_frame_size),
        }
    }

    /// Check if the framer has timed out waiting for more bytes
    ///
    /// Returns true if inter-frame timeout has elapsed since last byte
    pub fn is_frame_complete(&self) -> bool {
        if let Some(last_time) = self.last_byte_time {
            Instant::now().duration_since(last_time) > self.frame_timeout()
        } else {
            false
        }
    }

    /// Reset the framer state, discarding any accumulated bytes
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.last_byte_time = None;
    }

    /// Get the number of bytes currently buffered
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }
}

impl Default for RtuFramer {
    fn default() -> Self {
        Self::new()
    }
}

impl RtuFramer {
    /// Build an RTU request frame for sending
    ///
    /// Takes slave address, PDU, calculates CRC and returns complete frame
    pub fn build_frame(slave_address: u8, pdu: &ModbusPdu) -> Vec<u8> {
        let frame = ModbusRtuFrame {
            slave_address,
            pdu: pdu.clone(),
        };
        frame.encode()
    }

    /// Validate that a received frame has correct CRC
    pub fn validate_crc(data: &[u8]) -> Result<(), ModbusError> {
        if data.len() < MIN_RTU_FRAME_SIZE {
            return Err(ModbusError::IncompleteFrame {
                got: data.len(),
                expected: MIN_RTU_FRAME_SIZE,
            });
        }

        let data_len = data.len() - 2;
        let calculated_crc = ModbusRtuFrame::calculate_crc(&data[..data_len]);
        let frame_crc = u16::from_le_bytes([data[data_len], data[data_len + 1]]);

        if calculated_crc != frame_crc {
            return Err(ModbusError::CrcMismatch {
                expected: calculated_crc,
                got: frame_crc,
            });
        }

        Ok(())
    }

    /// Calculate the inter-byte timeout for a given baud rate
    /// Uses ceiling division: ceil(char_time * 1.5)
    pub fn calc_inter_char_timeout(baud_rate: u32) -> Duration {
        let char_time_ms: u64 = (10 * 1000) / baud_rate.max(1) as u64;
        // Ceiling: ((n * 3) + 1) / 2, then max(1)
        let timeout_ms = (((char_time_ms * 3) + 1) / 2).max(1u64);
        Duration::from_millis(timeout_ms)
    }

    /// Calculate the frame timeout for a given baud rate
    /// Uses ceiling division: ceil(char_time * 3.5)
    pub fn calc_frame_timeout(baud_rate: u32) -> Duration {
        let char_time_ms: u64 = (10 * 1000) / baud_rate.max(1) as u64;
        // Ceiling: ((n * 7) + 1) / 2, then max(4)
        let timeout_ms = (((char_time_ms * 7) + 1) / 2).max(4u64);
        Duration::from_millis(timeout_ms)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framer_new() {
        let framer = RtuFramer::new();
        assert_eq!(framer.buffer_len(), 0);
        assert_eq!(framer.baud_rate, 9600);
    }

    #[test]
    fn test_framer_with_baud_rate() {
        let framer = RtuFramer::with_baud_rate(115200);
        assert_eq!(framer.baud_rate, 115200);
    }

    #[test]
    fn test_char_time_calculation() {
        // At 9600 baud: 10 bits/char, char_time = 10000/9600 ≈ 1.04ms
        let framer = RtuFramer::with_baud_rate(9600);
        assert_eq!(framer.char_time_ms(), 1);

        // At 115200 baud: char_time = 10000/115200 ≈ 0.087ms → 0
        let framer = RtuFramer::with_baud_rate(115200);
        assert_eq!(framer.char_time_ms(), 0); // rounds to 0 for high baud
    }

    #[test]
    fn test_inter_char_timeout() {
        let framer = RtuFramer::with_baud_rate(9600);
        let timeout = framer.inter_char_timeout();
        // At 9600: 1.5 * 1.04ms ≈ 1.56ms, rounded up to 2ms
        assert_eq!(timeout, Duration::from_millis(2));
    }

    #[test]
    fn test_frame_timeout() {
        let framer = RtuFramer::with_baud_rate(9600);
        let timeout = framer.frame_timeout();
        // At 9600: 3.5 * 1.04ms ≈ 3.65ms, rounded up
        assert_eq!(timeout, Duration::from_millis(4));
    }

    #[test]
    fn test_push_byte_incomplete() {
        let mut framer = RtuFramer::new();
        let result = framer.push_byte(0x01);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert_eq!(framer.buffer_len(), 1);
    }

    #[test]
    fn test_push_byte_complete_frame() {
        let mut framer = RtuFramer::new();
        // Build a complete RTU frame: slave=1, FC=03, addr=0, count=10, CRC
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let frame = ModbusRtuFrame {
            slave_address: 1,
            pdu,
        };
        let encoded = frame.encode();

        let result = framer.push_bytes(&encoded);
        assert!(result.is_ok());
        let frames = result.unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].slave_address, 1);
    }

    #[test]
    fn test_framer_reset() {
        let mut framer = RtuFramer::new();
        framer.push_byte(0x01).unwrap();
        framer.push_byte(0x03).unwrap();
        assert_eq!(framer.buffer_len(), 2);

        framer.reset();
        assert_eq!(framer.buffer_len(), 0);
    }

    #[test]
    fn test_calc_inter_char_timeout() {
        let timeout = RtuFramer::calc_inter_char_timeout(9600);
        assert_eq!(timeout, Duration::from_millis(2));
    }

    #[test]
    fn test_calc_frame_timeout() {
        let timeout = RtuFramer::calc_frame_timeout(9600);
        assert_eq!(timeout, Duration::from_millis(4));
    }

    #[test]
    fn test_build_frame() {
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let frame_bytes = RtuFramer::build_frame(1, &pdu);

        // Should have: addr(1) + fc(1) + addr(2) + count(2) + CRC(2) = 8 bytes
        assert_eq!(frame_bytes.len(), 8);
        assert_eq!(frame_bytes[0], 0x01); // slave address
        assert_eq!(frame_bytes[1], 0x03); // function code

        // Validate CRC
        let result = RtuFramer::validate_crc(&frame_bytes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_crc_invalid() {
        let invalid_frame = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00];
        let result = RtuFramer::validate_crc(&invalid_frame);
        assert!(matches!(result, Err(ModbusError::CrcMismatch { .. })));
    }

    #[test]
    fn test_validate_crc_incomplete() {
        let short_frame = vec![0x01, 0x03];
        let result = RtuFramer::validate_crc(&short_frame);
        assert!(matches!(result, Err(ModbusError::IncompleteFrame { .. })));
    }

    #[test]
    fn test_framer_multiple_frames() {
        let mut framer = RtuFramer::new();

        // Build two consecutive frames
        let pdu1 = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let frame1 = ModbusRtuFrame {
            slave_address: 1,
            pdu: pdu1,
        };
        let encoded1 = frame1.encode();

        let pdu2 = ModbusPdu::write_single_register(100, 0x1234).unwrap();
        let frame2 = ModbusRtuFrame {
            slave_address: 1,
            pdu: pdu2,
        };
        let encoded2 = frame2.encode();

        // Concatenate two frames
        let mut combined = encoded1.clone();
        combined.extend_from_slice(&encoded2);

        let result = framer.push_bytes(&combined);
        assert!(result.is_ok());
        let frames = result.unwrap();
        assert_eq!(frames.len(), 2);
    }

    #[test]
    fn test_framer_exception_response() {
        let mut framer = RtuFramer::new();
        // Exception response: slave=1, FC=0x83, exception=0x02, CRC
        let exception_frame = vec![0x01, 0x83, 0x02, 0x90, 0x09];
        let result = framer.push_bytes(&exception_frame);
        assert!(result.is_ok());
        let frames = result.unwrap();
        assert_eq!(frames.len(), 1);
        assert!(frames[0].pdu.is_exception());
        assert_eq!(frames[0].pdu.get_exception_code(), Some(0x02));
    }

    #[test]
    fn test_calc_timeout_high_baud() {
        // At 115200: char_time = 10000/115200 ≈ 0.087 → 1ms minimum
        let timeout = RtuFramer::calc_inter_char_timeout(115200);
        assert_eq!(timeout, Duration::from_millis(1));
    }

    #[test]
    fn test_framer_is_frame_complete() {
        let framer = RtuFramer::new();
        // No bytes received yet
        assert!(!framer.is_frame_complete());
    }
}
