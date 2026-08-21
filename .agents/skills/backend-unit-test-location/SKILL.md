---
name: backend-unit-test-location
description: Use this skill when adding, moving, or reviewing `#[cfg(test)]` unit tests in a Rust `src/` file under `crates/`. Covers where the test code should physically live and how to keep it able to reach private items.
---

# Where Rust Unit Tests Live

## 1. What this is

Unit tests for a `src/<name>.rs` file do not live inline in that file. They live in a sibling `src/_tests/test_<name>.rs`, reached through a `#[path]`-redirected module declaration left in the original file — written with the `tests_file!` macro from the [`test_utils`](../../../crates/test_utils/README.md) crate. This keeps implementation files short without losing the thing an inline `mod tests` block gives you for free: access to private items.

This is a **pure module-tree fact**: Rust privacy is scoped by the module hierarchy, not by which physical file a module's code sits in. A `mod tests;` declared inside `foo.rs` is a child of `foo`'s module and can see everything private in it — regardless of where `#[path]` points its body. Splitting the file changes nothing about visibility.

**Scope**: this covers unit tests only — the `#[cfg(test)]` block that used to sit at the bottom of an implementation file. It does not apply to `crates/*/tests/` (integration tests, one binary per backend, run against a shared trait suite) — see [`backend-trait-test`](../backend-trait-test/SKILL.md) for those.

## 2. The pattern

For an implementation file `src/<name>.rs` that needs a `#[cfg(test)] mod tests { ... }`:

1. Write the test code in `src/_tests/test_<name>.rs` as normal top-level module content — no wrapping `mod tests { ... }`, no extra indentation. Start with `use super::*;` to pull in the parent module's items, followed by any other imports the tests need.
2. At the bottom of `src/<name>.rs`, in place of the inline block, add:
   ```rust
   test_utils::tests_file!("_tests/test_<name>.rs");
   ```
   which expands to the `#[cfg(test)]` + `#[path]` + `mod tests;` triple. The path is relative to the directory of the file invoking it, exactly as a hand-written `#[path]` would be. The trailing semicolon is required — a macro invoked with `()` in item position needs one. Invoke it fully qualified: nothing else is added anywhere, this one line is the whole declaration.
3. If the crate does not use `tests_file!` yet, add the dependency. It has to be a regular `[dependencies]` entry, not a dev-dependency: the invocation sits outside any `cfg`, so it must resolve in non-test builds too, where it expands to nothing.
   ```toml
   test_utils = { path = "../test_utils" }
   ```
4. If several files in the same directory have tests, they share one `_tests/` directory — one `test_<name>.rs` per implementation file, each declared independently from its own `tests_file!` line. No `_tests/mod.rs` is needed; `#[path]` makes each declaration self-contained.
5. Drop any `TESTS` banner comment that used to precede the inline block — the `tests_file!` line is signal enough.
6. The module is always named `tests` (plural) — the macro fixes that name, so a file being converted from `mod test` (singular) is normalized for free.
7. An `#[allow(...)]` or other attribute that sat on the original `mod tests { ... }` goes inside the invocation, before the path. It applies to the whole module regardless of where the body physically lives — do not duplicate it into the new file.
   ```rust
   test_utils::tests_file!(
       #[allow(clippy::field_reassign_with_default)]
       "_tests/test_config.rs"
   );
   ```

## 3. Do and don't

| Do | Don't |
|---|---|
| Put test code in `src/_tests/test_<name>.rs`, addressed via `tests_file!` | Leave a `#[cfg(test)] mod tests { ... }` block inline in the implementation file |
| Declare it with `test_utils::tests_file!("_tests/test_<name>.rs");` | Hand-write the `#[cfg(test)]` / `#[path]` / `mod tests;` triple the macro replaces |
| Keep the invocation fully qualified — one line, no crate-root boilerplate | Add `#[macro_use] extern crate test_utils;` or a `use` to shorten the call |
| Start the new file with `use super::*;` | Re-import every private item by hand |
| Name the file after the module it tests: `gen_patch.rs` → `test_gen_patch.rs` | Use a generic name like `tests.rs` when a directory holds more than one implementation file |
| Keep one `_tests/` directory per parent directory, shared by every implementation file in it | Nest a `_tests/` per file |
| Touch only `src/` | Reorganize anything under `crates/*/tests/` — that's integration-test territory, owned by [`backend-trait-test`](../backend-trait-test/SKILL.md) |
| Relocate a commented-out test block as-is (still commented, same content) if it's disabled pending a dependency that doesn't exist yet | Uncomment, "fix", or otherwise touch a disabled test block while relocating it |

## 4. Checklist

- [ ] No implementation file under `crates/*/src/` still has an inline `#[cfg(test)] mod tests { ... }` (or `mod test { ... }`) block — check with `grep -rn "mod test" --include=*.rs crates/*/src | grep -v _tests`.
- [ ] No hand-written `#[path = "_tests/...` triple is left — check with `grep -rn 'path = "_tests' --include=*.rs crates/`.
- [ ] Every `_tests/test_<name>.rs` starts with `use super::*;`.
- [ ] `crates/*/tests/` (integration tests) was not touched.
- [ ] `cargo fmt --all`
- [ ] `cargo clippy --workspace --all-targets --all-features`
- [ ] `cargo test -p <crate> --lib` — same pass/fail counts as before the move (this is a pure reorganization, never a behavior change).
