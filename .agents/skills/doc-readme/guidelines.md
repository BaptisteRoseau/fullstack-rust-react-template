# README guidelines

The standard every `README.md` in this repository follows, extracted from the existing ones.
When a rule here and an existing README disagree, the rule wins: the older READMEs
(`infrastructure/README.md`, `infrastructure/cicd/README.md`, `infrastructure/configs/README.md`)
predate it.

## 1. What a README is for

The repository has three kinds of documentation, and a README is only one of them:

| Question                                                | Where it is answered            |
| ------------------------------------------------------- | ------------------------------- |
| How does the system fit together?                       | [doc/](../../../doc/README.md)  |
| What belongs in this directory, and what may it import? | That directory's `README.md`    |
| How do I perform this task, step by step?               | A skill under `.claude/skills/` |

A README answers four questions about **its own directory**, and nothing else:

1. What is this directory, in one or two sentences?
2. What is inside it, and where does a new file go?
3. What rules hold here, and why?
4. What traps does someone changing it fall into?

It does **not** hold step-by-step procedures. Those go in a skill, and the README links to it:

> See the [backend-trait-test](../backend-trait-test/SKILL.md) skill for the step-by-step: the
> suite file, one file per backend, the `Cargo.toml` stanza and the `tests/README.md` that goes
> with them.

Agents read every README from the working directory up to the root before touching code (see the
root `CLAUDE.md`). So a README is written for a reader who already read its parents, and never
repeats them.

A README is read by **both humans and agents**: - keep it straight to the point. - avoid long paragraphs - use simple and clear vocabulary

## 2. Skeleton

Sections appear in this order. Only the title and the opening paragraph are mandatory; include
any other section only when the directory has something to say there.

```txt
# <Title>

<Opening paragraph: what this is.>

<Optional second paragraph: where it sits, what it is not, link to the parent rule.>

## Public surface        crates and modules: the items other code uses
## Directory             the tree, with a comment per entry
## Conventions           how things are named and shaped here
## <Topic sections>      one per concept worth explaining: "Why two structs", "The Keycloak backend"
## Environment           variables the directory reads
## Secrets               where secrets live, when the directory touches any
## Running               the command to run its tests or itself
## Verification          the script that checks a change here
## Rules                 absolute rules, see section 6
## Skills                always last
```

Synonyms already in use and accepted: `## Layout` or `## Structure` for `## Directory`,
`## Validation` for `## Verification`.

### Title

- A crate or top-level directory gets a human name: `# App Core`, `# Production Nodes`,
  `# Database CRUD Derive`.
- A frontend `src/` folder keeps its folder name in lowercase: `# api`, `# components`, `# e2e`.
- A crate whose name is an identifier keeps it: `# test_trait`.
- A `tests/` directory is `# <Crate> integration tests`.

### Opening paragraph

No heading above it. Start with a noun phrase that names what the directory **is**, then say
what it does or the one question it answers:

- "The HTTP layer. Turns `app_core` and the service crates into an Axum server: routing,
  request/response models, extractors, middlewares and the OpenAPI document."
- "Roles, scopes and permission checks. Answers one question: can this user do this thing to
  this object?"

Do not open with "This directory contains…" or "Here lies…": the reader knows they are in a
directory.

The second paragraph, when present, places the directory among its neighbours. Say what it is
**not** and why, and point to the parent README for a shared rule instead of restating it:

- "This is not a service crate: it exposes plain functions, not a trait, because there is only
  one implementation and no backend to swap."
- "See `crates/README.md` for the service-crate shape this crate follows."

## 3. Section patterns

### Public surface

A bullet list of what other code imports, each item linked to its file, followed by an em dash
and what it is for:

```md
- [`init_logger(debug: bool, json: bool)`](src/lib.rs) — call once, at startup.
- [`error::CoreError`](src/error.rs) — the crate-wide error enum. See the error convention
  in [crates/README.md](../README.md).
```

Use a table instead when every item has the same columns (a trait's methods and their
responsibility, a module and its contents).
### Directory tree

A fenced `txt` block, drawn with box characters, the root written as `<dir>/`, a `#` comment
on each entry aligned in one column, and `<placeholders>` in angle brackets for repeated
shapes:

```txt
api/
├── src/
│   ├── app_state.rs   # AppState and its FromRef impls
│   ├── endpoints/     # route handlers, one directory per resource — see endpoints/README.md
│   └── error/         # ApiError and its HTTP response conversion
└── Cargo.toml
```

- Show the shape a new file must follow, not every file that exists. A directory with many
  similar children shows one `<name>/` entry.
- The comment says what lives there, or the one rule about it (`— never edited by hand`,
  `GENERATED — do not hand-edit`, `the only file naming generated *types*`).
- Follow the tree with prose for whatever the comments cannot carry.
- Avoid putting **every** files when not necessary. This tree shows the structure, not an exhaustive list.

Prefer:

```txt
endpoints/
├── <domain>/
    └── routes.rs   # The available routes for <domain>
```

Over:

```txt
endpoints/
├── users/
│   └── routes.rs   # The available routes for users
├── keys/
│   └── routes.rs   # The available routes for keys
├── files/
    └── routes.rs   # The available routes for files
```

### Conventions

Each convention is one short paragraph or bullet starting with the rule in **bold**, then the
reason or an example:

```md
- **One object per file, named after its kind**, in the singular: `deployment.yaml`,
  `service.yaml`. The directory already carries the component name — never repeat it in the
  file name.
```

```md
**`lib.rs` and `mod.rs` hold no logic.** Only `mod`, `use` and `pub use`. Real code goes in a
new or existing file.
```

Show the canonical form in a code block when a rule is about code shape, as `crates/README.md`
does for the error enum and `crates/config/README.md` does for a `DEFAULT_*` constant.

### Topic sections

One `##` per concept a reader needs before changing things, titled after the concept:
`## Why two structs`, `## The main.rs contract`, `## The Garage fixture`,
`## The errors are the contract`. Lead into a list of traps with a sentence that says they are
traps:

- "Notes worth knowing before changing anything here:"
- "Two details make this work and are easy to break:"
- "Three things in `start_container()` are load-bearing and easy to mistake for ceremony:"

### Running and Verification

The exact command, alone in a code block, then one sentence on what it needs or proves:

````md
## Running

```sh
cargo test -p authenticator
```

Requires a running Docker daemon; the Keycloak container takes a few seconds to boot.
````

When several commands share a block, comment each one on its line:

```bash
./scripts/build_frontend_api_sdk.sh    # regenerate; needs cargo and bun
./scripts/test_openapi.sh              # fails if the committed SDK no longer matches the router
```

### Skills

Always the last section. One bullet per skill relevant to the directory, linked through the
`.claude` symlink with a path relative to the README:

```md
## Skills

- [backend-trait-test](../../../.claude/skills/backend-trait-test/SKILL.md)
- [backend-feature-gating](../../../.claude/skills/backend-feature-gating/SKILL.md)
```

Omit the section when no skill applies.

## 4. Writing style

- **Plain words, short sentences**, as if explaining to a junior engineer. Present tense.
- **Say why.** A rule without its reason gets broken. Prefer "so", "because" and a colon:
  "`main` owns the `Config` and passes it into `run`. Keeping it at the top level lets it
  outlive `run`, so the whole program can read it without lifetime plumbing."
- **Name the failure mode**, especially a silent one: "a bare `cargo test -p database`
  silently skips it", "the suite still compiles, still passes, and quietly covers less than it
  claims", "an undocumented response is a response the frontend cannot see".
- **Be concrete.** Name the real file, type, flag, port or command instead of describing it.
  Give one real example from the codebase rather than an invented one: "For `GET /user/{uuid}`
  the response type is `GetUserResponse`."
- **Point to the smallest complete example**: "`crates/cache/tests` is the smallest complete
  example."
- **State the real status honestly**: "**Not implemented.**", "not yet wired into
  `has_access`", "Nothing below has been run yet". Do not document what does not exist as if
  it did.
- **Bold** carries hard rules and key terms on first use (`**library**`, `**never**`,
  `**Do not edit anything in this folder.**`). Do not bold whole paragraphs for emphasis.
- `**WARNING**:` at the start of a paragraph is reserved for a state that breaks things.
- Use the em dash `—` with spaces for an aside or an item description.
- Put every identifier, path, command, flag, variable and value in backticks.
- No emojis, no marketing tone, no "simply", no "just", no "easy".
- Say a thing once. If the parent README or a skill already says it, link to it.

## 5. Formatting

These match `.markdownlint.json` and the newer READMEs.

- Wrap prose at **100 characters**. Never break inside a link or inline code.
- One blank line around headings, lists, tables and code blocks.
- `-` for bullets, `1.` for ordered steps. Nested bullets are indented by **4 spaces**.
- Every code block has a language: `txt` for trees and diagrams, `sh` or `bash` for shell
  (match the rest of the file), `rust`, `tsx`, `yaml`, `sql`, `dockerfile`, `nix`, `md`.
- Tables use the compact separator `| --- | --- |`; do not pad columns by hand.
- Bare URLs are wrapped in angle brackets: `<http://127.0.0.1:3000>`.
- Links are relative to the README (`../config`, `src/cache.rs`, `../../.claude/skills/...`).
  Link text is the file name, the crate name or the item, never "here" or "this link".
- Link a directory as `[cache](./cache)` or to its README as `[crates/README.md](../README.md)`;
  link a code item to its file as ``[`Cache`](src/cache.rs)``.
- Escape a literal `<` in link text: `[\<root>/.env.dev](../../.env.dev)`.

## 6. The Rules section

The root `CLAUDE.md` makes every `## Rules` section **absolute** for its directory and all its
children. Only put a rule there when breaking it is never acceptable.

- Bullets, each one checkable: "Using `app_core` or `api` crates is forbidden here",
  "Never commit a secret, an SSH private key or a join token."
- No reasoning inside the bullet beyond a short clause; the reasoning belongs in the
  `## Conventions` or topic section above.
- A soft preference goes in `## Conventions`, not here.

## 7. Size

Fit the README to the directory. A leaf folder or small crate is 15–40 lines: title, opening
paragraph, a tree or public surface, and skills. A directory that owns a convention other
directories follow (`crates/`, `infrastructure/kubernetes/`, `frontend/docs/architecture/`) can
run to 200 lines. Past that, move the procedures into a skill and the deep dives into `doc/`.

## 8. Keeping READMEs true

A README describes the code as it is now.

- After renaming, moving or deleting a file, a type or a command, grep the READMEs for it.
- A new child directory with its own README is added to the parent's tree with a
  `— see <dir>/README.md` comment.
- A new skill is added to the `## Skills` section of every README whose directory it applies to.
