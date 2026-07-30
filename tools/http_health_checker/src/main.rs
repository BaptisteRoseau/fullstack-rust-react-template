use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let endpoint = args.last().unwrap();
    match minreq::get(endpoint).with_timeout(3).send() {
        Ok(response) => ExitCode::from((response.status_code > 299) as u8),
        Err(_) => ExitCode::from(1),
    }
}
