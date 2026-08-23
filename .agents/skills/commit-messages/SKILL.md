---
name: commit-messages
description: Use when writing a commit message or preparing a commit in this repository.
---

# Write a commit message

Generate consistent, informative commit messages following the Conventional Commits specification.

## 1. Read the change

```bash
git diff --staged
```

Commit only what you staged. Never `git add .`.

## 2. Write the subject

```txt
<type>(<scope>): <description>
```

| Type | Description | Example |
| --- | --- | --- |
| `feat` | New feature | `feat(auth): add OAuth2 login` |
| `fix` | Bug fix | `fix(api): handle null response` |
| `docs` | Documentation only | `docs(readme): add setup instructions` |
| `style` | Formatting, no code change | `style: fix indentation` |
| `refactor` | Code change, no new feature/fix | `refactor(db): extract query builder` |
| `perf` | Performance improvement | `perf(search): add result caching` |
| `test` | Adding/fixing tests | `test(auth): add login unit tests` |
| `build` | Build system changes | `build: update webpack config` |
| `ci` | CI configuration | `ci: add GitHub Actions workflow` |
| `chore` | Maintenance tasks | `chore(deps): update dependencies` |
| `ai` | AI agent related | `ai(skill): add the ai type in commit-messages` |
| `revert` | Revert previous commit | `revert: feat(auth): add OAuth2` |

The scope is a noun naming the area: a crate (`config`, `api`), `frontend`, `scripts`, `repo`.
Omit it when the change is genuinely broad.

Rules for the description:

- Imperative mood: "add", never "added" or "adds".
- Lowercase after the colon, no full stop at the end.
- Keep the whole subject line under 72 characters.

## 3. Add a body when the reason is not obvious

Separate it from the subject with a blank line and wrap at 72 characters. Say why the change was
needed and what it makes possible. Use bullets for several related changes.

## 4. Add footers when they apply

- `Closes #123` to close an issue, `Refs #456` to reference one.
- `BREAKING CHANGES:` (always plural) followed by one line per break.

A change to the HTTP API contract — **removing**, **renaming** or **changing the type of** a field
or enum key — must be listed with an `API:` prefix. See
[endpoints/README.md](../../../crates/api/src/endpoints/README.md).

```txt
update(api): return the user's groups from /auth/me

The frontend needs group membership to decide which nav entries to
render, and was fetching it separately on every page load.

BREAKING CHANGES:
API: renamed GetMeResponse.roles to GetMeResponse.groups
```

## Checklist

- [ ] The subject says why, not a restatement of the diff.
- [ ] The type is one from the table above.
- [ ] An API contract change carries a `BREAKING CHANGES:` footer with an `API:` line.
