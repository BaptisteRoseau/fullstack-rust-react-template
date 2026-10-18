---
name: ai-skill-redaction
description: Use when writing a new skill, or changing, splitting, renaming or reviewing an existing one under .agents/skills (.claude/skills).
---

# Write a skill

Every skill follows one standard, written down in [guidelines.md](./guidelines.md). **Read it in
full before writing**, then follow the steps below.

A skill teaches one task. If what you are writing describes a directory rather than a task, it is a
README — Skill(doc-readme-redaction).

## 1. Collect what the agent lacks

Before writing a line, gather the material the skill exists to carry:

- the READMEs of the directories the task touches, so the skill links to them instead of
  repeating them;
- the files a correct result changes, and one existing example to copy;
- the mistakes an agent makes on this task without the skill — the corrections you had to give are
  the skill's most valuable lines;
- the existing checks that prove the task is done: linters, `scripts/test_*.sh`, `bun run *:check`.

## 2. Name it and write the description

```bash
mkdir .agents/skills/<prefix>-<task>
```

Pick the prefix from the table in [guidelines.md](./guidelines.md#2-name) (`frontend-`,
`backend-`, `infrastructure-`, `doc-`, `ai-`, `api-`, or a new one for a new area). The directory
name and the `name` field are the same string.

Write the description before the body: one line, starting with `Use when`, listing the user's
intents, the paths and the failing checks that should load it.

## 3. Write the body

Follow the skeleton in [guidelines.md](./guidelines.md#5-skillmd-body) and the closest model skill
listed there: a verb-phrase title, a short opening, numbered imperative steps with real code from
the repository, the gotchas next to the step they concern, `## Checklist` last.

Move any branch most runs do not need into a `<topic>.md` next to `SKILL.md`, linked with the
condition to read it.

## 4. Script every mechanical check

For each rule in the skill, ask whether a program could check it. If yes, it must not stay prose:

1. An existing tool already checks it: put the command in the checklist.
2. It holds across the repository: write or extend a `scripts/test_*.sh`, so the `pre-push` hook
   runs it.
3. It is about the result of this task: write `scripts/check_<thing>.sh` in the skill, following
   [guidelines.md](./guidelines.md#writing-a-check-script), and `chmod +x` it.

When you change a rule in an existing skill, change its script in the same edit.

## 5. Wire it in

- Add the skill to the `## Skills` section of every README whose directory it applies to.
- For a frontend skill, add it to the `Agents:` line of `frontend/README.md`.
- Add a one-line rule to the root `CLAUDE.md` only if the skill must run on every occurrence of the
  task, whatever the wording.

## 6. Try it

Run a real task with the skill loaded and read the trace. Check that a prompt worded the way a user
would — without the skill's name — loads it, and that a near-miss does not. Turn every correction
into a gotcha or a script.

## Checklist

```bash
.claude/skills/ai-skill-redaction/scripts/check_skill.sh <name>
git add .agents/skills/<name> && ./scripts/test_lint_doc.sh
```

`check_skill.sh` covers the frontmatter, the name, the description, the length, the checklist
position, the `Skill(...)` references, the reference files and the scripts. `test_lint_doc.sh`
covers the Markdown style and every relative link; it only sees tracked files, hence the
`git add`.

- [ ] Every line is something the agent would get wrong without it.
- [ ] Every rule a program could check is checked by a script or an existing tool.
- [ ] The steps use real files and code from the repository, and name the example to copy.
- [ ] The skill is linked from the `## Skills` section of the READMEs it applies to.
