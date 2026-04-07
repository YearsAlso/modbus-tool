//! Script management commands

use crate::commands::CommandResponse;
use crate::error::{Error, Result};
use modbus_tool_lib::{Action, Script, ScriptEngine, ScriptStatus, Trigger};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

/// Global script engine instance
static SCRIPT_ENGINE: once_cell::sync::Lazy<Arc<Mutex<ScriptEngine>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(ScriptEngine::new())));

// ========== Tauri Commands ==========

/// List all scripts
#[tauri::command]
pub fn script_list() -> CommandResponse<Vec<Script>> {
    match script_list_inner() {
        Ok(scripts) => CommandResponse::ok(scripts),
        Err(e) => {
            error!("Failed to list scripts: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_list_inner() -> Result<Vec<Script>> {
    let engine = SCRIPT_ENGINE.lock();
    Ok(engine.get_all_scripts().into_iter().cloned().collect())
}

/// Get a single script by ID
#[tauri::command]
pub fn script_get(id: String) -> CommandResponse<Option<Script>> {
    match script_get_inner(&id) {
        Ok(script) => CommandResponse::ok(script),
        Err(e) => {
            error!("Failed to get script {}: {}", id, e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_get_inner(id: &str) -> Result<Option<Script>> {
    let uuid = Uuid::parse_str(id).map_err(|_| Error::Parse(format!("Invalid UUID: {}", id)))?;
    let engine = SCRIPT_ENGINE.lock();
    Ok(engine.get_script(&uuid).cloned())
}

/// Create or update a script
#[tauri::command]
pub fn script_save(script: Script) -> CommandResponse<Script> {
    match script_save_inner(script) {
        Ok(saved) => {
            info!("Script saved: {}", saved.id);
            CommandResponse::ok(saved)
        }
        Err(e) => {
            error!("Failed to save script: {}", e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_save_inner(mut script: Script) -> Result<Script> {
    // Ensure updated_at is set
    script.updated_at = chrono::Utc::now();
    
    let mut engine = SCRIPT_ENGINE.lock();
    engine.add_script(script.clone());
    Ok(script)
}

/// Delete a script
#[tauri::command]
pub fn script_delete(id: String) -> CommandResponse<bool> {
    match script_delete_inner(&id) {
        Ok(deleted) => {
            if deleted {
                info!("Script deleted: {}", id);
            }
            CommandResponse::ok(deleted)
        }
        Err(e) => {
            error!("Failed to delete script {}: {}", id, e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_delete_inner(id: &str) -> Result<bool> {
    let uuid = Uuid::parse_str(id).map_err(|_| Error::Parse(format!("Invalid UUID: {}", id)))?;
    let mut engine = SCRIPT_ENGINE.lock();
    Ok(engine.remove_script(&uuid).is_some())
}

/// Get script execution status
#[tauri::command]
pub fn script_status(id: String) -> CommandResponse<Option<ScriptStatus>> {
    match script_status_inner(&id) {
        Ok(status) => CommandResponse::ok(status),
        Err(e) => {
            error!("Failed to get script status {}: {}", id, e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_status_inner(id: &str) -> Result<Option<ScriptStatus>> {
    let uuid = Uuid::parse_str(id).map_err(|_| Error::Parse(format!("Invalid UUID: {}", id)))?;
    let engine = SCRIPT_ENGINE.lock();
    Ok(engine.get_status(&uuid).cloned())
}

/// Start a script (mark as running)
#[tauri::command]
pub fn script_start(id: String) -> CommandResponse<bool> {
    match script_start_inner(&id) {
        Ok(_) => {
            info!("Script started: {}", id);
            CommandResponse::ok(true)
        }
        Err(e) => {
            error!("Failed to start script {}: {}", id, e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_start_inner(id: &str) -> Result<()> {
    let uuid = Uuid::parse_str(id).map_err(|_| Error::Parse(format!("Invalid UUID: {}", id)))?;
    let mut engine = SCRIPT_ENGINE.lock();
    if let Some(status) = engine.statuses.get_mut(&uuid) {
        status.running = true;
    }
    Ok(())
}

/// Stop a script (mark as not running)
#[tauri::command]
pub fn script_stop(id: String) -> CommandResponse<bool> {
    match script_stop_inner(&id) {
        Ok(_) => {
            info!("Script stopped: {}", id);
            CommandResponse::ok(true)
        }
        Err(e) => {
            error!("Failed to stop script {}: {}", id, e);
            CommandResponse::err(e.code(), e.to_string())
        }
    }
}

fn script_stop_inner(id: &str) -> Result<()> {
    let uuid = Uuid::parse_str(id).map_err(|_| Error::Parse(format!("Invalid UUID: {}", id)))?;
    let mut engine = SCRIPT_ENGINE.lock();
    if let Some(status) = engine.statuses.get_mut(&uuid) {
        status.running = false;
    }
    Ok(())
}

/// Get all script statuses
#[tauri::command]
pub fn script_list_statuses() -> CommandResponse<HashMap<String, ScriptStatus>> {
    let engine = SCRIPT_ENGINE.lock();
    let statuses: HashMap<String, ScriptStatus> = engine
        .statuses
        .iter()
        .map(|(id, status)| (id.to_string(), status.clone()))
        .collect();
    CommandResponse::ok(statuses)
}
