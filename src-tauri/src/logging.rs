//! Logging module for Modbus Tool

use std::path::PathBuf;

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

/// Initialize logging system
pub fn init_logging(log_dir: Option<PathBuf>) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,modbus_tool=debug"));

    // Console layer - always enabled in debug
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .compact();

    // File layer - only if log_dir is provided
    if let Some(log_path) = log_dir {
        std::fs::create_dir_all(&log_path)?;
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            &log_path,
            "modbus-tool.log",
        );

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(console_layer)
            .with(file_layer)
            .init();
    } else {
        // Console only
        tracing_subscriber::registry()
            .with(env_filter)
            .with(console_layer)
            .init();
    }

    tracing::info!("Logging initialized");
    Ok(())
}
