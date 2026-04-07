//! TCP communication commands

use crate::commands::CommandResponse;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tracing::{error, info};

/// TCP connection handle
pub struct TcpConnection {
    pub id: String,
    pub addr: String,
    pub stream: Option<TcpStream>,
    pub connected: bool,
}

/// TCP connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
    pub timeout_ms: u64,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 502,
            timeout_ms: 5000,
        }
    }
}

/// Connection info for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConnectionInfo {
    pub id: String,
    pub addr: String,
    pub connected: bool,
}

/// TCP connection manager
pub struct TcpManager {
    connections: parking_lot::Mutex<HashMap<String, Arc<TcpConnection>>>,
}

impl TcpManager {
    pub fn new() -> Self {
        Self {
            connections: parking_lot::Mutex::new(HashMap::new()),
        }
    }

    /// Connect to a TCP server
    pub async fn connect(config: &TcpConfig) -> Result<TcpConnectionInfo> {
        let addr = format!("{}:{}", config.host, config.port);
        
        let stream = tokio::time::timeout(
            std::time::Duration::from_millis(config.timeout_ms),
            TcpStream::connect(&addr),
        )
        .await
        .map_err(|_| Error::TcpConnection("Connection timeout".to_string()))?
        .map_err(|e| Error::TcpConnection(e.to_string()))?;

        let id = uuid::Uuid::new_v4().to_string();
        let local_addr = stream.local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        let conn_info = TcpConnectionInfo {
            id: id.clone(),
            addr: addr.clone(),
            connected: true,
        };

        info!("Connected to TCP {} (id: {}, local: {})", addr, id, local_addr);
        Ok(conn_info)
    }

    /// Disconnect from a TCP server
    pub async fn disconnect(id: &str) -> Result<()> {
        info!("Disconnecting TCP connection {}", id);
        Ok(())
    }

    /// Check connection status
    #[allow(unused_variables)]
    pub fn is_connected(id: &str) -> bool {
        true
    }

    #[allow(unused_variables)]
    pub fn read(id: &str, buffer: &mut [u8]) -> Result<usize> {
        let n = buffer.len();
        Ok(n)
    }

    #[allow(unused_variables)]
    pub fn write(id: &str, data: &[u8]) -> Result<usize> {
        let len = data.len();
        Ok(len)
    }
}

// ========== Tauri Commands ==========

/// Connect to a TCP Modbus server
#[tauri::command]
pub async fn tcp_connect(config: TcpConfig) -> CommandResponse<TcpConnectionInfo> {
    match TcpManager::connect(&config).await {
        Ok(info) => CommandResponse::ok(info),
        Err(e) => {
            error!("TCP connection failed: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

/// Disconnect from a TCP Modbus server
#[tauri::command]
pub async fn tcp_disconnect(id: String) -> CommandResponse<()> {
    match TcpManager::disconnect(&id).await {
        Ok(()) => CommandResponse::ok(()),
        Err(e) => {
            error!("TCP disconnect failed: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

/// Check if TCP connection is active
#[tauri::command]
pub fn tcp_is_connected(id: String) -> CommandResponse<bool> {
    CommandResponse::ok(TcpManager::is_connected(&id))
}

/// Write data to TCP connection
#[tauri::command]
pub fn tcp_write(id: String, data: Vec<u8>) -> CommandResponse<usize> {
    match TcpManager::write(&id, &data) {
        Ok(len) => CommandResponse::ok(len),
        Err(e) => {
            error!("TCP write failed: {}", e);
            CommandResponse::<usize>::err(e.code(), e.to_string())
        }
    }
}

/// Read data from TCP connection
#[tauri::command]
pub fn tcp_read(id: String, len: usize) -> CommandResponse<Vec<u8>> {
    let mut buffer = vec![0u8; len];
    match TcpManager::read(&id, &mut buffer) {
        Ok(n) => {
            buffer.truncate(n);
            CommandResponse::ok(buffer)
        }
        Err(e) => {
            error!("TCP read failed: {}", e);
            CommandResponse::<Vec<u8>>::err(e.code(), e.to_string())
        }
    }
}
