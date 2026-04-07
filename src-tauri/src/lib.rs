//! Modbus Tool - Rust Backend Library
//!
//! Modern Modbus Protocol Debug Tool built with Tauri 2.0 + Rust

pub mod error;
pub mod logging;
pub mod commands;
pub mod modbus;
pub mod storage;

pub use error::{Error, Result};
pub use logging::init_logging;

// Re-export modbus protocol items for convenience
pub use modbus::protocol::*;
