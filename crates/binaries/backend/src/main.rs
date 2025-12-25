use api;
use config;
use database;
mod program;
use std::process::exit;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::parse()?;
    if let Err(error) = program::run(&config).await {
        eprintln!("Fatal Error: {}", error);
        exit(1);
    }
    Ok(())
}

