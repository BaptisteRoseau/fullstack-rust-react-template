# Binaries

This is where all the crates that implement a binary should be. They are the only ones allowed to have a `main.rs` file.

Each `main.rs` should look like the following with at least a `program.rs` alongside it:

```rs
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
```

This allows the config to live longer than `program::run` hence be readable in the whole program without lifetime issue.

Binary crates should be minimal. If something is big enough to end up in its own or an existing crate, create it or update existing crates.
