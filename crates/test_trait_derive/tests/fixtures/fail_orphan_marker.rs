//! A marker outside a suite module: the test would never be collected.
use test_trait_derive::test_trait;

trait Subject {}

#[test_trait]
async fn never_collected(subject: &impl Subject) {
    let _ = subject;
}

fn main() {}
