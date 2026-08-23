# test_utils

Test helpers shared by the crates that need them. It carries no runtime code: it exports
macros only, so depending on it costs nothing in a release build.

## `tests_file!`

Declares the `#[path]`-redirected `mod tests;` that points a file's unit tests at a
sibling `src/_tests/test_<name>.rs`. See the
[backend-unit-test-location](../../.claude/skills/backend-unit-test-location/SKILL.md)
skill for where that file goes and how to write it; [`tests_file.rs`](src/tests_file.rs)
is the macro itself.

It is a regular `[dependencies]` entry, not a dev-dependency, because the invocation sits
outside any `cfg` and must resolve in non-test builds too, where it expands to nothing.

## Skills

- [backend-unit-test-location](../../.claude/skills/backend-unit-test-location/SKILL.md)
