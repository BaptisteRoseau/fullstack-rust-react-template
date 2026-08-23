# test_trait_derive

The proc-macro half of [test_trait](../test_trait). Nothing depends on this crate
directly: `test_trait` re-exports the three macros so a consumer has one
dev-dependency.

## Why it exists

A trait suite that lists its own tests in a hand-written `Vec<Trial>` names every test
twice — once in its signature, once in the collector. Forgetting the second mention
drops the test from the run and **nothing reports it**: the suite still compiles, still
passes, and quietly covers less than it claims.

So the module is the single source of truth. `#[test_trait_suite]` reads it and
generates the collector, and every way of writing a suite that would collect nothing is
a compile error instead.

## The three macros

| Macro | Applies to | Effect |
| --- | --- | --- |
| `#[test_trait]` | an `async fn` inside a suite module | marker only — no transform |
| `#[test_trait_suite]` | an inline `mod { … }` | appends `trials` and, when possible, `trials_shared` |
| `test_trait_main!(Fixture)` | a path expression | writes the `harness = false` binary's `fn main()` |

### `#[test_trait]` — the marker

It distinguishes tests from **async helpers** living in the same module. It performs no
transform of its own: the enclosing module's attribute expands first and rewrites every
marker it collects into `#[test_trait(collected)]`, which this macro passes through
untouched.

The rewrite, rather than a removal, buys two things: the attribute still resolves
through the user's `use test_trait::test_trait;` so that import does not look unused,
and a marker still bare by the time it expands can only mean the test sits outside a
suite module — reported as an error rather than silently never running.

### `#[test_trait_suite]` — what it gathers

Given the module body, it walks the items and collects every `Item::Fn` carrying
`#[test_trait]`. Anything else — helpers, `const`s, `use`, nested types — is left
exactly as written.

From each collected function it reads:

| Read from | Used for |
| --- | --- |
| the function name | the trial name passed to `Trial::test` |
| `async` | required; the collector awaits the call |
| the first parameter's type | the subject: its trait bounds, and how to pass it |
| the first parameter's form | `&T` → `&subject`, `&mut T` → `&mut subject`, `T` → `subject` |
| the second parameter, if any | the context; must be `&`, since it is shared by every trial |

The subject's bounds are taken from `impl Trait` or `dyn Trait` and become the
generated functions' `S: …` bound. Every test declaring a subject — or a context — must
name the same one: one suite drives one trait.

The generated items are appended to the module:

```rust
pub fn trials<S: Cache, B: Fn() -> F, F: Future<Output = S>>(rt: Arc<Runtime>, build: B) -> Vec<Trial>;
pub fn trials_shared<S: Cache + Send + Sync + 'static>(rt: Arc<Runtime>, subject: Arc<S>) -> Vec<Trial>;
```

When any test declares a context, both gain a `C: Ctx + Send + Sync + 'static`
parameter and a trailing `context: Arc<C>` argument; only the tests that asked for it
are handed it.

`trials_shared` is generated **only** when every test takes its subject by shared
reference — a `&mut` or by-value subject cannot come out of an `Arc`. Its doc comment
carries the caveat that the trials run in parallel against that one subject.

### `test_trait_main!` — the binary

Takes the path of a type implementing `test_trait::TestSuite` and expands to a `main`
that parses libtest arguments, builds the tokio runtime, `block_on`s `start()`, collects
`trials()`, runs them, and exits with the conclusion.

The teardown is the part worth generating: the fixture is dropped inside `rt.enter()`,
because `ContainerAsync::Drop` spawns its cleanup asynchronously and outside the runtime
that cleanup silently cannot run — leaking containers onto the developer's machine.

## The errors are the contract

Each of these has a fixture in `tests/fixtures/` pinning the exact message, checked by
`trybuild`:

| Fixture | Rejected because |
| --- | --- |
| `fail_orphan_marker` | `#[test_trait]` outside a suite module — the test would never be collected |
| `fail_empty_suite` | a suite module with no `#[test_trait]` — it would run nothing |
| `fail_file_module` | `mod suite;` instead of `mod suite { … }` — no body to read |
| `fail_no_subject` | a test taking no parameter — there is no subject to run against |
| `fail_not_async` | a non-`async` test — the collector awaits it |
| `fail_concrete_subject` | `&S3` instead of `&impl Storage` — pins the suite to one backend |
| `fail_mismatched_subjects` | two tests naming different traits — a suite drives one |

`fail_concrete_subject` is the one worth understanding. A concrete subject compiles,
passes, and reads almost identically to the right thing; the suite just quietly stops
being a trait suite, and nothing surfaces it until a second backend arrives and the
"reusable" suite has to be rewritten. That is exactly what happened to `crates/database`
before this check existed.

## Layout

```txt
src/
├── lib.rs      # the three #[proc_macro*] entry points and their docs
├── suite.rs    # #[test_trait_suite] parsing and code generation, plus the marker
└── main_fn.rs  # test_trait_main! expansion
tests/
├── trybuild.rs # compile_fail over tests/fixtures/*.rs
└── fixtures/   # one .rs / .stderr pair per rejected way of writing a suite
```

## Running

```sh
cargo test -p test_trait_derive
```

Fixture messages are pinned character for character. After changing an error, review the
diff `trybuild` prints and refresh the `.stderr` with:

```sh
TRYBUILD=overwrite cargo test -p test_trait_derive
```

## Skills

- [backend-trait-test](../../.claude/skills/backend-trait-test/SKILL.md)
