//! Exports the backend OpenAPI document to a JSON file.
//!
//! The document is built directly from the `api` crate router, so no server,
//! database, cache or storage backend needs to be running. This is the offline
//! counterpart to fetching `/v1/docs/openapi.json` from a live backend.
//!
//! Usage: `cargo run -p openapi_generator -- [OUTPUT_PATH]`
//! (defaults to `openapi.json`).

use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_OUTPUT: &str = "openapi.json";

fn main() -> ExitCode {
    let output = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT));

    if let Err(error) = export(&output) {
        eprintln!("Failed to export the OpenAPI document: {error}");
        return ExitCode::FAILURE;
    }

    println!("Wrote the OpenAPI document to {}", output.display());
    ExitCode::SUCCESS
}

fn export(output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let json = api::routes::openapi().to_pretty_json()?;

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(output, format!("{json}\n"))?;

    Ok(())
}
