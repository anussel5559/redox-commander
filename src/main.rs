#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Context;
use redox_core::util::{paths, ResultTraced};
use redox_tui::Tui;
use std::{
    fs::{self, File, OpenOptions},
    io,
    process::ExitCode,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    filter::Targets, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    initialize_tracing(false);
    info!("Starting Redox Commander");
    Tui::start().await;
    Ok(ExitCode::SUCCESS)
}

/// Set up tracing to a log file, and optionally the console as well. If there's
/// an error creating the log file, we'll skip that part. This means in the TUI
/// the error (and all other tracing) will never be visible, but that's a
/// problem for another day.
fn initialize_tracing(console_output: bool) {
    // Failing to log shouldn't be a fatal crash, so just move on
    let log_file = initialize_log_file()
        .context("Error creating log file")
        .traced()
        .ok();

    // Basically a minimal version of EnvFilter that doesn't require regexes
    // https://github.com/tokio-rs/tracing/issues/1436#issuecomment-918528013
    let targets: Targets = std::env::var("RUST_LOG")
        .ok()
        .and_then(|env| env.parse().ok())
        .unwrap_or_else(|| {
            Targets::new()
                .with_target("rc", LevelFilter::INFO)
                .with_target("redox_core", LevelFilter::INFO)
                .with_target("redox_tui", LevelFilter::INFO)
                .with_target("redox_api", LevelFilter::DEBUG)
        });

    let file_subscriber = log_file.map(|log_file| {
        // Include PID
        // https://github.com/tokio-rs/tracing/pull/2655
        tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_writer(log_file)
            .with_target(false)
            .with_ansi(false)
            .with_span_events(FmtSpan::NEW)
            .with_filter(targets)
    });

    // Enable console output for CLI
    let console_subscriber = if console_output {
        Some(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stderr)
                .with_target(false)
                .with_span_events(FmtSpan::NEW)
                .without_time()
                .with_filter(LevelFilter::WARN),
        )
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(console_subscriber)
        .init();
}

/// Create the log file. If it already exists, make sure it's not over a max
/// size. If it is, move it to a backup path and nuke whatever might be in the
/// backup path.
fn initialize_log_file() -> anyhow::Result<File> {
    const MAX_FILE_SIZE: u64 = 1000 * 1000; // 1MB
    let path = paths::log_file();
    paths::create_parent(&path)?;

    if fs::metadata(&path).map_or(false, |metadata| metadata.len() > MAX_FILE_SIZE) {
        // Rename new->old, overwriting old. If that fails, just delete new so
        // it doesn't grow indefinitely. Failure shouldn't stop us from logging
        // though
        let _ = fs::rename(&path, paths::log_file_old()).or_else(|_| fs::remove_file(&path));
    }

    let log_file = OpenOptions::new().create(true).append(true).open(path)?;
    Ok(log_file)
}
