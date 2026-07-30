//! The macros' error messages are part of their contract: a suite that silently
//! collects nothing is the failure mode they exist to prevent, so every way of
//! writing one by accident should say so.

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fixtures/*.rs");
}
