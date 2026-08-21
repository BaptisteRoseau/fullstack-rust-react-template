# test_utils

Test helpers shared by every backend crate. It carries no runtime code: it
exports macros only, so depending on it costs nothing in a release build.

## `tests_file!`

Unit tests do not live inline in the implementation file, they live in a sibling
`src/_tests/test_<name>.rs` reached through a `#[path]`-redirected module — see
the `backend-unit-test-location` skill for the rationale. `tests_file!` is the
one-line spelling of that declaration:

```rust
test_utils::tests_file!("_tests/test_scope.rs");
```

instead of:

```rust
#[cfg(test)]
#[path = "_tests/test_scope.rs"]
mod tests;
```

The path is relative to the directory of the file invoking the macro, exactly as
with a hand-written `#[path]`.

An attribute that would have sat on the `mod tests;` declaration goes before the
path, inside the invocation:

```rust
test_utils::tests_file!(
    #[allow(clippy::field_reassign_with_default)]
    "_tests/test_config.rs"
);
```

## Using it from a crate

The invocation is fully qualified, so it needs nothing at the crate root — just
the dependency:

```toml
# Cargo.toml
[dependencies]
test_utils = { path = "../test_utils" }
```

It is a regular dependency rather than a dev-dependency because the invocation
sits outside any `cfg` and must therefore resolve in non-test builds too, where
it expands to nothing.
