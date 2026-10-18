---
name: doc-readme-redaction
description: Use when writing a new README.md, or restructuring or reviewing an existing one, anywhere in the repository.
---

# Write a README

Every directory's `README.md` follows one standard, written down in
[guidelines.md](./guidelines.md). **Read it in full before writing**, then follow the steps below.

## 1. Read what the reader already read

Read the README of every parent directory, up to the root. Your README is read after them, so it
must not repeat them: link to the parent for any rule it already states.

Read one or two sibling READMEs of the same kind as a model:

| Writing a README for | Model it on |
| --- | --- |
| A service crate (trait + backends) | [crates/cache/README.md](../../../crates/cache/README.md) |
| A plain crate (functions, no trait) | [crates/compressor/README.md](../../../crates/compressor/README.md) |
| A crate's `tests/` directory | [crates/storage/tests/README.md](../../../crates/storage/tests/README.md) |
| A frontend `src/` folder | [frontend/src/pages/README.md](../../../frontend/src/pages/README.md) |
| An infrastructure directory | [infrastructure/kubernetes/README.md](../../../infrastructure/kubernetes/README.md) |

## 2. Read the directory itself

List the files, read the entry points (`lib.rs`, `index.ts`, `Cargo.toml`, `kustomization.yaml`,
`Dockerfile`), and grep for who uses it. Every claim in the README must be checked against the
code: a type name, a command, a port, a status ("not implemented", "not yet wired").

## 3. Write it

Follow the skeleton in [guidelines.md](./guidelines.md#2-skeleton): title, opening paragraph, then
only the sections the directory has something to say in, `## Skills` last.

Keep step-by-step procedures out of it. If the directory needs one, it belongs in a skill under
`.claude/skills/`, and the README links to that skill.

Add a `## Rules` section only for rules that are never acceptable to break: the root `CLAUDE.md`
makes them absolute for this directory and every child.

## 4. Wire it in

- Add the new directory to the parent README's tree, with a `— see <dir>/README.md` comment when
  the tree has comments.
- List every skill that applies to the directory in its `## Skills` section, with a path relative
  to the README through `.claude/skills/`.

## Checklist

```bash
./scripts/test_lint_doc.sh    # markdownlint and markdown-link-check over every tracked .md
```

The link check only sees tracked files: `git add` the new README before running it.

- [ ] The opening paragraph says what the directory is, without "This directory contains".
- [ ] Nothing from a parent README is restated; it is linked instead.
- [ ] Every rule says why, and every silent failure mode is named.
- [ ] Every name, path and command in the README exists in the code today.
- [ ] Prose wraps at 100 characters, code blocks have a language, nested bullets use 4 spaces.
- [ ] `## Skills` is the last section, or absent when no skill applies.
