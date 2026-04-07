//! Modbus Tool - Tauri Application Entry Point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use modbus_tool_lib::{commands, init_logging};

fn main() {
    // Initialize logging
    if let Err(e) = init_logging(None) {
        eprintln!("Failed to initialize logging: {}", e);
    }

    tracing::info!("Starting Modbus Tool...");

    // Build and run Tauri application
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Serial port commands
            commands::serial::serial_list_ports,
            commands::serial::serial_open_port,
            commands::serial::serial_close_port,
            commands::serial::serial_write,
            commands::serial::serial_read,
            // TCP commands
            commands::tcp::tcp_connect,
            commands::tcp::tcp_disconnect,
            commands::tcp::tcp_is_connected,
            commands::tcp::tcp_write,
            commands::tcp::tcp_read,
        ])
        .run(tauri::generate_context!());

    if let Err(e) = result {
        tracing::error!("Application error: {}", e);
        std::process::exit(1);
    }
}
