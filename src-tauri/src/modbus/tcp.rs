//! Modbus TCP Protocol Implementation
//!
//! Handles Modbus TCP connection state, MBAP transaction management,
//! and protocol state tracking.

use crate::modbus::protocol::{MbapHeader, ModbusError, ModbusPdu, ModbusTcpFrame, DEFAULT_PORT};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};

/// Modbus TCP connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connection in progress
    Connecting,
    /// Successfully connected
    Connected,
    /// Connection error
    Error(String),
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Disconnected => write!(f, "Disconnected"),
            ConnectionState::Connecting => write!(f, "Connecting"),
            ConnectionState::Connected => write!(f, "Connected"),
            ConnectionState::Error(s) => write!(f, "Error: {}", s),
        }
    }
}

/// Connection metadata for Modbus TCP
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Remote socket address
    pub remote_addr: SocketAddr,
    /// Local socket address
    pub local_addr: SocketAddr,
    /// Connection state
    pub state: ConnectionState,
    /// Last activity time (epoch millis)
    pub last_activity: u64,
    /// Number of bytes sent
    pub bytes_sent: u64,
    /// Number of bytes received
    pub bytes_received: u64,
}

impl ConnectionInfo {
    pub fn new(remote_addr: SocketAddr, local_addr: SocketAddr) -> Self {
        Self {
            remote_addr,
            local_addr,
            state: ConnectionState::Connected,
            last_activity: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
}

/// Transaction ID manager for Modbus TCP
///
/// Manages unique transaction IDs for matching requests with responses.
/// Each request gets a unique ID, and the response carries the same ID.
#[derive(Debug)]
pub struct TransactionManager {
    /// Next transaction ID (wraps around at 65535)
    next_id: AtomicU16,
    /// Pending transactions: transaction_id -> (request_time, expected_response_len)
    pending: parking_lot::RwLock<HashMap<u16, Transaction>>,
    /// Maximum pending transactions (to prevent memory exhaustion)
    max_pending: usize,
}

/// A pending transaction entry
#[derive(Debug, Clone)]
struct Transaction {
    /// Request PDU
    pdu: ModbusPdu,
    /// Unit ID from the request
    unit_id: u8,
    /// Request timestamp (epoch millis)
    request_time: u64,
    /// Expected response length (MBAP length field)
    expected_len: usize,
}

impl TransactionManager {
    /// Create a new transaction manager
    pub fn new() -> Self {
        Self {
            next_id: AtomicU16::new(1),
            pending: parking_lot::RwLock::new(HashMap::new()),
            max_pending: 100,
        }
    }

    /// Generate a new unique transaction ID
    pub fn next_transaction_id(&self) -> u16 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Register a new pending transaction
    ///
    /// Returns the transaction ID assigned to this request.
    pub fn begin_transaction(
        &self,
        pdu: ModbusPdu,
        unit_id: u8,
        expected_response_len: usize,
    ) -> u16 {
        let id = self.next_transaction_id();

        let transaction = Transaction {
            pdu,
            unit_id,
            request_time: Self::current_epoch_ms(),
            expected_len: expected_response_len,
        };

        // Clean up old pending transactions if we're at capacity
        {
            let mut pending = self.pending.write();
            if pending.len() >= self.max_pending {
                // Collect (tid, request_time) pairs sorted by time, keep newest 50%
                let mut entries: Vec<(u16, u64)> = pending
                    .iter()
                    .map(|(tid, t)| (*tid, t.request_time))
                    .collect();
                entries.sort_unstable_by_key(|&(_, time)| time);
                // Remove oldest 50%
                let to_remove: Vec<u16> = entries
                    .into_iter()
                    .take(pending.len() / 2)
                    .map(|(tid, _)| tid)
                    .collect();
                for tid in to_remove {
                    pending.remove(&tid);
                }
            }
            pending.insert(id, transaction);
        }

        id
    }

    /// Complete a transaction (response received)
    pub fn end_transaction(&self, transaction_id: u16) -> Option<Transaction> {
        self.pending.write().remove(&transaction_id)
    }

    /// Get transaction by ID
    pub fn get_transaction(&self, transaction_id: u16) -> Option<Transaction> {
        self.pending.read().get(&transaction_id).cloned()
    }

    /// Get number of pending transactions
    pub fn pending_count(&self) -> usize {
        self.pending.read().len()
    }

    /// Clear all pending transactions
    pub fn clear(&self) {
        self.pending.write().clear();
    }

    /// Get current epoch milliseconds
    fn current_epoch_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Modbus TCP session state
///
/// Holds all state for a Modbus TCP connection including the transport
/// state, transaction management, and connection metadata.
#[derive(Debug)]
pub struct ModbusTcpSession {
    /// Connection information
    pub connection: Option<ConnectionInfo>,
    /// Transaction manager
    pub transactions: TransactionManager,
    /// Default unit ID (slave address for serial passthrough)
    pub default_unit_id: u8,
    /// TCP port (default 502)
    pub port: u16,
}

impl ModbusTcpSession {
    /// Create a new TCP session
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new TCP session with specified default unit ID
    pub fn with_unit_id(unit_id: u8) -> Self {
        Self {
            connection: None,
            transactions: TransactionManager::new(),
            default_unit_id: unit_id,
            port: DEFAULT_PORT,
        }
    }

    /// Set connection info (after successful connect)
    pub fn set_connected(&mut self, remote: SocketAddr, local: SocketAddr) {
        self.connection = Some(ConnectionInfo::new(remote, local));
    }

    /// Set connection state to disconnected
    pub fn set_disconnected(&mut self) {
        if let Some(ref mut conn) = self.connection {
            conn.state = ConnectionState::Disconnected;
        }
        self.transactions.clear();
    }

    /// Set connection state to error
    pub fn set_error(&mut self, error: String) {
        if let Some(ref mut conn) = self.connection {
            conn.state = ConnectionState::Error(error);
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connection
            .as_ref()
            .map(|c| c.state == ConnectionState::Connected)
            .unwrap_or(false)
    }

    /// Build a TCP request frame with fresh transaction ID
    pub fn build_request(&self, pdu: &ModbusPdu) -> ModbusTcpFrame {
        let transaction_id = self.transactions.next_transaction_id();
        let length = 1 + pdu.data.len() as u16; // unit_id + pdu

        ModbusTcpFrame {
            mbap: MbapHeader {
                transaction_id,
                protocol_id: 0,
                length,
                unit_id: self.default_unit_id,
            },
            pdu: pdu.clone(),
        }
    }

    /// Register a request and return its transaction ID
    pub fn register_request(&self, pdu: &ModbusPdu) -> u16 {
        // MBAP length = 1 (unit_id) + 1 (function_code) + data.len()
        let expected_response_len = 1 + 1 + 1; // minimum: unit_id + fc + byte_count
        self.transactions.begin_transaction(pdu.clone(), self.default_unit_id, expected_response_len)
    }

    /// Parse a response and validate it against pending transactions
    pub fn parse_response(&self, data: &[u8]) -> Result<ModbusTcpFrame, ModbusError> {
        let frame = ModbusTcpFrame::parse(data)?;

        // Validate transaction ID matches a pending request
        let transaction = self.transactions.get_transaction(frame.mbap.transaction_id);
        if transaction.is_none() {
            // Transaction ID not found - could be old or unsolicited
            // We still return the frame but caller should handle
            tracing::warn!(
                "Unexpected transaction ID: {}",
                frame.mbap.transaction_id
            );
        }

        Ok(frame)
    }

    /// Complete a transaction after receiving response
    pub fn complete_transaction(&self, transaction_id: u16) -> Option<Transaction> {
        self.transactions.end_transaction(transaction_id)
    }
}

impl Default for ModbusTcpSession {
    fn default() -> Self {
        Self {
            connection: None,
            transactions: TransactionManager::new(),
            default_unit_id: 0xFF, // 0xFF is common for TCP broadcast/initial
            port: DEFAULT_PORT,
        }
    }
}

/// MBAP helper utilities
impl MbapHeader {
    /// Get the MBAP header size in bytes
    pub const MBAP_SIZE: usize = 7;

    /// Validate MBAP header
    pub fn validate(&self) -> Result<(), ModbusError> {
        if self.protocol_id != 0 {
            return Err(ModbusError::MbapParseError(format!(
                "Invalid protocol_id: {}, expected 0 for Modbus",
                self.protocol_id
            )));
        }

        // Length must be at least 1 (unit_id) + 1 (function_code)
        if self.length < 2 {
            return Err(ModbusError::MbapParseError(format!(
                "Invalid length: {}, minimum is 2",
                self.length
            )));
        }

        Ok(())
    }

    /// Check if a complete frame is received given the total buffer size
    pub fn is_complete(&self, buffer_size: usize) -> bool {
        let required = 7 + (self.length - 1) as usize;
        buffer_size >= required
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_state_display() {
        assert_eq!(format!("{}", ConnectionState::Disconnected), "Disconnected");
        assert_eq!(format!("{}", ConnectionState::Connecting), "Connecting");
        assert_eq!(format!("{}", ConnectionState::Connected), "Connected");
        assert_eq!(
            format!("{}", ConnectionState::Error("timeout".to_string())),
            "Error: timeout"
        );
    }

    #[test]
    fn test_transaction_manager_new() {
        let tm = TransactionManager::new();
        assert_eq!(tm.pending_count(), 0);
    }

    #[test]
    fn test_transaction_manager_next_id() {
        let tm = TransactionManager::new();
        let id1 = tm.next_transaction_id();
        let id2 = tm.next_transaction_id();
        assert_eq!(id2, id1 + 1);
    }

    #[test]
    fn test_transaction_manager_begin_end() {
        let tm = TransactionManager::new();
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();

        let tid = tm.begin_transaction(pdu.clone(), 1, 5);
        assert_eq!(tm.pending_count(), 1);

        let tx = tm.end_transaction(tid);
        assert!(tx.is_some());
        assert_eq!(tm.pending_count(), 0);
    }

    #[test]
    fn test_transaction_manager_get() {
        let tm = TransactionManager::new();
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();

        let tid = tm.begin_transaction(pdu.clone(), 1, 5);
        let tx = tm.get_transaction(tid);
        assert!(tx.is_some());
        assert_eq!(tx.unwrap().unit_id, 1);
    }

    #[test]
    fn test_transaction_manager_clear() {
        let tm = TransactionManager::new();
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        tm.begin_transaction(pdu.clone(), 1, 5);
        assert_eq!(tm.pending_count(), 1);

        tm.clear();
        assert_eq!(tm.pending_count(), 0);
    }

    #[test]
    fn test_mbap_header_validate() {
        let mbap = MbapHeader {
            transaction_id: 1,
            protocol_id: 0,
            length: 6,
            unit_id: 1,
        };
        assert!(mbap.validate().is_ok());
    }

    #[test]
    fn test_mbap_header_validate_invalid_protocol() {
        let mbap = MbapHeader {
            transaction_id: 1,
            protocol_id: 1,
            length: 6,
            unit_id: 1,
        };
        assert!(mbap.validate().is_err());
    }

    #[test]
    fn test_mbap_header_validate_invalid_length() {
        let mbap = MbapHeader {
            transaction_id: 1,
            protocol_id: 0,
            length: 1, // too short, minimum is 2
            unit_id: 1,
        };
        assert!(mbap.validate().is_err());
    }

    #[test]
    fn test_mbap_is_complete() {
        let mbap = MbapHeader {
            transaction_id: 1,
            protocol_id: 0,
            length: 6, // 7 header + 5 PDU = 12 bytes total
            unit_id: 1,
        };
        assert!(!mbap.is_complete(5));
        assert!(mbap.is_complete(12));
    }

    #[test]
    fn test_mbap_size_constant() {
        assert_eq!(MbapHeader::MBAP_SIZE, 7);
    }

    #[test]
    fn test_session_new() {
        let session = ModbusTcpSession::new();
        assert!(session.connection.is_none());
        assert_eq!(session.default_unit_id, 0xFF);
        assert_eq!(session.port, 502);
    }

    #[test]
    fn test_session_with_unit_id() {
        let session = ModbusTcpSession::with_unit_id(5);
        assert_eq!(session.default_unit_id, 5);
    }

    #[test]
    fn test_session_build_request() {
        let session = ModbusTcpSession::with_unit_id(1);
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let frame = session.build_request(&pdu);

        assert_eq!(frame.mbap.protocol_id, 0);
        assert_eq!(frame.mbap.unit_id, 1);
        assert_eq!(frame.mbap.length, 6); // 1 + 5 bytes of PDU data
    }

    #[test]
    fn test_session_register_request() {
        let session = ModbusTcpSession::with_unit_id(1);
        let pdu = ModbusPdu::read_holding_registers(0, 10).unwrap();
        let tid = session.register_request(&pdu);
        assert_eq!(session.transactions.pending_count(), 1);

        session.complete_transaction(tid);
        assert_eq!(session.transactions.pending_count(), 0);
    }

    #[test]
    fn test_session_is_connected() {
        let session = ModbusTcpSession::new();
        assert!(!session.is_connected());
    }

    #[test]
    fn test_mbap_const_size() {
        // MBAP is always 7 bytes
        let data = [0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x01];
        assert_eq!(data.len(), MbapHeader::MBAP_SIZE);
        let mbap = MbapHeader::parse(&data).unwrap();
        assert_eq!(mbap.transaction_id, 1);
    }

    #[test]
    fn test_session_default_port() {
        assert_eq!(DEFAULT_PORT, 502);
    }
}
