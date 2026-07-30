//! Macros that assemble a trait test suite at compile time.
//!
//! The problem they solve: a suite that lists its own tests in a hand-written
//! `Vec<Trial>` names every test twice, and forgetting the second mention drops
//! the test from the run without any error. Here the module is the single source
//! of truth — [`macro@trait_test_suite`] reads it and generates the collector.

use proc_macro::TokenStream;

mod main_fn;
mod suite;

/// Marks one test inside a [`macro@trait_test_suite`] module.
///
/// A marker, not a transform: the enclosing module's attribute expands first and
/// uses these to tell tests from helpers. It leaves behind a marker carrying an
/// argument, which this macro passes through untouched. A bare marker therefore
/// means the test sits outside a suite module and would never be collected, which
/// is worth an error rather than silence.
#[proc_macro_attribute]
pub fn trait_test(args: TokenStream, input: TokenStream) -> TokenStream {
    suite::marker(args.into(), input.into()).into()
}

/// Generates a suite's trial collector from the `#[trait_test]` functions in a module.
///
/// Adds `trials(rt, build)`, which builds a fresh subject per trial, and — when
/// every test takes its subject by shared reference — `trials_shared(rt, subject)`,
/// which runs them all against one instance. Both return `Vec<Trial>` for
/// `libtest-mimic`.
///
/// The subject type and the way it is passed are read from the tests' first
/// parameter, so `&impl Trait`, `&mut impl Trait`, a concrete type and by-value
/// forms all work without extra configuration. A second parameter, if any test
/// declares one, becomes a context argument on the generated functions.
#[proc_macro_attribute]
pub fn trait_test_suite(_args: TokenStream, input: TokenStream) -> TokenStream {
    suite::expand(input.into()).into()
}

/// Generates the `fn main()` of a `harness = false` test binary.
///
/// Takes the path to a fixture implementing `test_utils::TestSuite`: it starts the
/// fixture, collects its trials, runs them, and tears the fixture down inside the
/// runtime context so async container cleanup can complete.
///
/// ```ignore
/// test_utils::trait_test_main!(common::containers::GarageFixture);
/// ```
#[proc_macro]
pub fn trait_test_main(input: TokenStream) -> TokenStream {
    main_fn::expand(input.into()).into()
}
