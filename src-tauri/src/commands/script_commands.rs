//! Tauri commands for script management

use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::script::{Action, Script, ScriptEngine, Trigger};
use crate::AppState;

/// Get all scripts
#[tauri::command]
pub fn script_list(state: State<'_, Mutex<AppState>>) -> Result<Vec<Script>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state.script_engine.get_all_scripts().into_iter().cloned().collect())
}

/// Get a single script by ID
#[tauri::command]
pub fn script_get(id: String, state: State<'_, Mutex<AppState>>) -> Result<Option<Script>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    Ok(state.script_engine.get_script(&uuid).cloned())
}

/// Save a script (create or update)
#[tauri::command]
pub fn script_save(script: Script, state: State<'_, Mutex<AppState>>) -> Result<Script, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    
    // Check if script exists
    if state.script_engine.get_script(&script.id).is_some() {
        // Update: remove and re-add
        state.script_engine.remove_script(&script.id);
    }
    
    // Add the script
    state.script_engine.add_script(script.clone());
    
    Ok(script)
}

/// Delete a script
#[tauri::command]
pub fn script_delete(id: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    state.script_engine.remove_script(&uuid);
    Ok(())
}

/// Get script status
#[tauri::command]
pub fn script_status(id: String, state: State<'_, Mutex<AppState>>) -> Result<Option<crate::script::ScriptStatus>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    Ok(state.script_engine.get_status(&uuid).cloned())
}

/// Start a script (set enabled = true)
#[tauri::command]
pub fn script_start(id: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    if let Some(script) = state.script_engine.get_script_mut(&uuid) {
        script.enabled = true;
        Ok(())
    } else {
        Err("Script not found".to_string())
    }
}

/// Stop a script (set enabled = false)
#[tauri::command]
pub fn script_stop(id: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    if let Some(script) = state.script_engine.get_script_mut(&uuid) {
        script.enabled = false;
        Ok(())
    } else {
        Err("Script not found".to_string())
    }
}

/// Evaluate all scripts with current register values
/// Returns IDs of triggered scripts
#[tauri::command]
pub fn script_evaluate(registers: HashMap<String, u16>, state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let triggered = state.script_engine.evaluate(&registers);
    Ok(triggered.into_iter().map(|u| u.to_string()).collect())
}

/// Get all script statuses
#[tauri::command]
pub fn script_list_statuses(state: State<'_, Mutex<AppState>>) -> Result<Vec<crate::script::ScriptStatus>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let scripts = state.script_engine.get_all_scripts();
    let mut statuses = Vec::new();
    
    for script in scripts {
        if let Some(status) = state.script_engine.get_status(&script.id) {
            statuses.push(status.clone());
        }
    }
    
    Ok(statuses)
}
