//! Tauri commands module

pub mod serial;
pub mod tcp;
pub mod script;

use serde::{Deserialize, Serialize};

/// Command response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse<T: serde::Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<CommandError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    pub code: i32,
    pub message: String,
}

impl<T: serde::Serialize> CommandResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(code: i32, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(CommandError { code, message }),
        }
    }
}
