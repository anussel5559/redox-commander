#![forbid(unsafe_code)]
#![deny(clippy::all)]

use redox_tui::Tui;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    Tui::start().await?;
    Ok(ExitCode::SUCCESS)
}
