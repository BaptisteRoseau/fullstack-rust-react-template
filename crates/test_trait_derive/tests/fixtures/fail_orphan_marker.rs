//! A marker outside a suite module: the test would never be collected.
use test_trait_derive::test_trait;

#[test_trait]
async fn never_collected(subject: &u8) {
    let _ = subject;
}

fn main() {}
