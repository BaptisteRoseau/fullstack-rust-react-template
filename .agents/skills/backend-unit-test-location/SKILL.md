---
name: backend-unit-test-location
description: Use when adding, moving or reviewing a #[cfg(test)] unit test in a Rust src/ file under crates/.
---

# Where Rust unit tests live

Unit tests do **not** sit inline in the implementation file. They live in a sibling
`src/_tests/test_<name>.rs`, declared from the original file with the `tests_file!` macro.

This costs nothing in visibility. Rust privacy follows the **module tree**, not the file layout. A
`mod tests;` declared inside `foo.rs` is a child of `foo` and sees everything private in it, no
matter where `#[path]` puts the body.

This covers unit tests only. For `crates/*/tests/`, use Skill(backend-trait-test).

## 1. Move the test code

Put it in `src/_tests/test_<name>.rs` as plain top-level content: no wrapping `mod tests { … }`, no
extra indentation. Start with `use super::*;` to reach the parent module's private items.

Name the file after the module it tests: `gen_patch.rs` becomes `test_gen_patch.rs`. One `_tests/`
directory per source directory, one file per implementation file.

## 2. Declare it

At the bottom of `src/<name>.rs`, replace the inline block with one line:

```rust
test_utils::tests_file!("_tests/test_<name>.rs");
```

Keep it fully qualified. Do not add `#[macro_use]` or a `use` to shorten it. The trailing semicolon
is required. The path is relative to the file invoking the macro.

An attribute that sat on the old `mod tests { … }` goes **inside** the invocation, before the path.
Do not copy it into the new file:

```rust
test_utils::tests_file!(
    #[allow(clippy::field_reassign_with_default)]
    "_tests/test_config.rs"
);
```

## 3. Add the dependency if the crate lacks it

```toml
[dependencies]
test_utils = { path = "../test_utils" }
```

It must be a regular dependency, not a dev-dependency: the invocation sits outside any `cfg`, so it
has to resolve in non-test builds too, where it expands to nothing. See
[test_utils](../../../crates/test_utils/README.md).

## Notes

- The module is always named `tests`, plural. The macro fixes the name, so a file being converted
  from `mod test` is normalised for free.
- Drop any `TESTS` banner comment. The `tests_file!` line is signal enough.
- Move a commented-out test block **as-is**, still commented. Do not uncomment or repair it while
  relocating.
- Never reorganise `crates/*/tests/` here.

## Checklist

```bash
.claude/skills/backend-unit-test-location/scripts/check_test_location.sh
cargo test -p <crate> --lib
```

The move is a pure reorganisation. `cargo test` must report the **same** pass and fail counts as
before it.
