//! Expansion of `trait_test_main!`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse2};

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let fixture: Path = match parse2(input) {
        Ok(path) => path,
        Err(error) => return error.to_compile_error(),
    };

    quote! {
        fn main() {
            let args = ::test_utils::Arguments::from_args();

            let rt = ::std::sync::Arc::new(
                ::test_utils::Runtime::new().expect("failed to build the tokio runtime"),
            );
            let fixture = ::std::sync::Arc::new(
                rt.block_on(<#fixture as ::test_utils::TestSuite>::start()),
            );

            let trials = ::test_utils::TestSuite::trials(
                ::std::sync::Arc::clone(&fixture),
                ::std::sync::Arc::clone(&rt),
            );
            let conclusion = ::test_utils::run(&args, trials);

            // Drop the fixture inside the runtime context: container handles run
            // their cleanup asynchronously on drop, and outside the runtime that
            // cleanup silently cannot execute, leaking containers.
            let guard = rt.enter();
            ::std::mem::drop(fixture);
            ::std::mem::drop(guard);
            ::std::mem::drop(rt);

            conclusion.exit();
        }
    }
}
