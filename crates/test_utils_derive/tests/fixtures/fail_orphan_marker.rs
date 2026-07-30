//! A marker outside a suite module: the test would never be collected.
use test_utils_derive::trait_test;

#[trait_test]
async fn never_collected(subject: &u8) {
    let _ = subject;
}

fn main() {}
