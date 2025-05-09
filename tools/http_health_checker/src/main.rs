use std::{env, process::ExitCode};

#[inline]
fn run(endpoint: &str) -> Result<minreq::Response, minreq::Error> {
    minreq::get(endpoint).with_timeout(3).send()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let endpoint = args.last().unwrap();
    let res = run(endpoint);
    if res.is_err() {
        return ExitCode::from(1);
    }
    let code = res.unwrap().status_code;
    if code > 299 {
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_reach_google() {
        let res = run("https://google.com");
        assert!(res.is_ok())
    }

    #[test]
    fn cant_reach_nonsense() {
        let res = run("https://weqweqwe.local/qwewqe");
        assert!(res.is_err())
    }
}
