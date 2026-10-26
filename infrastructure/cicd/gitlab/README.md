# GitLab CI

The GitLab CI pipeline. [gitlab-ci.yml](./gitlab-ci.yml) is its entry point, included by the
root [.gitlab-ci.yml](../../../.gitlab-ci.yml), and it includes every other file here.

The checks it runs and when they run are shared with every provider, see
[cicd/README.md](../README.md). This README covers how the GitLab manifests implement them.

## Directory

```txt
gitlab/
├── gitlab-ci.yml                 # stages, workflow rules, and the includes below
├── rules/
│   └── <category>.gitlab-ci.yml  # hidden rules shared by several jobs: .frontend-changed
├── templates/
│   ├── <tool>.gitlab-ci.yml      # hidden jobs, such as .rust or .docker-image
│   └── inputs/
│       └── <name>.gitlab-ci.yml  # templates with spec:inputs, included by the jobs using them
└── jobs/
    └── <category>.gitlab-ci.yml  # the jobs that run, one file per category of cicd/README.md
```

**Every manifest ends in `.gitlab-ci.yml`.** The includes and `test_lint_cicd.sh` select files by
that suffix, so a manifest named `<name>.yml` is silently left out of the pipeline and of the
lint.

- **`templates/`** holds hidden jobs, whose names start with a dot. Each one implements how a
  kind of job runs, for example how to build and push a Docker image.
- **`templates/inputs/`** holds templates that take [`spec:inputs`](#templates-with-inputs). The
  entry point's `templates/*.gitlab-ci.yml` wildcard does not reach this folder, on purpose: a
  wildcard include passes no inputs, so the pipeline would fail on the missing ones.
- **`jobs/`** holds the jobs that actually run. Each one extends a template and names what it
  applies to, for example `app-backend-image`.
- **`rules/`** holds a rule only once two jobs or templates need it. A rule used by a single job
  stays in that job.

## Rules

Templates and jobs can both define `rules`. A template sets the default trigger for its kind of
job. A job that needs another trigger replaces it, because `extends` **replaces** `rules`
instead of merging them.

A shared rule is a hidden key named after what changed, holding a `rules` list. A job lists it
next to the paths only it needs:

```yaml
app-keycloak-image:
  extends: .docker-image
  rules:
    - !reference [.check-all-files, rules]
    - !reference [.tmp-health-checker-image-changed, rules]
    - changes:
        paths:
          - infrastructure/docker/app_keycloak/**/*
        compare_to: refs/heads/$CI_DEFAULT_BRANCH
```

A job runs when any one of its rules matches, so splitting a path list across several rules does
not change when it runs.

## Changed files

The workflow in [gitlab-ci.yml](./gitlab-ci.yml) sets `CHECK_ALL_FILES` to `true` wherever every
check must run on every file. `.check-all-files` comes first in every job, so it then matches
before any `changes` rule.

`changes` rules compare to the default branch (`compare_to: refs/heads/$CI_DEFAULT_BRANCH`).
Variables in `compare_to` need GitLab 17.2 or later. On an older instance they are not expanded,
and the jobs no longer start on a branch.

A job that lints changed files only uses
[lint_changed_files.gitlab-ci.yml](./templates/inputs/lint_changed_files.gitlab-ci.yml), see
below.

## Templates with inputs

A template that a job configures takes `spec:inputs` instead of variables. A missing or
mistyped input stops the pipeline from being created, with the input's name in the error. A
misspelled variable is silently ignored instead.

Such a template generates a hidden job named after the `job` input. The job file includes the
template once per job, with its inputs, then extends the generated job:

```yaml
include:
  - local: infrastructure/cicd/gitlab/templates/inputs/lint_changed_files.gitlab-ci.yml
    inputs:
      job: shellcheck
      pathspecs: "*.sh scripts/git_hooks/pre-push"
      command: shellcheck

shellcheck:
  extends: .shellcheck-changed-files
  image: koalaman/shellcheck-alpine:v0.11.0
```

The inputs of `lint_changed_files.gitlab-ci.yml`:

- `job` — the name of the job, which extends `.<job>-changed-files`.
- `pathspecs` — the git pathspecs of the files to lint. Empty matches every file.
- `all_on_delete` — lint every matching file when the branch deleted or renamed a file, for a
  check that file can break, such as a markdown link.
- `command` — the command that receives the files.

## Cancelled and waiting jobs

- **A new commit cancels the older pipelines of its branch**, whole. GitLab only does so while
  no job with `interruptible: false` has started, and that is a job's default. `default:` sets
  `interruptible: true` for every job: **never set it to `false`**, or the first such job that
  starts keeps its whole pipeline running. The project setting "Auto-cancel redundant
  pipelines" must stay enabled.
- **A cancelled image job can leave some of its tags pushed**, such as the commit's tag without
  `latest`. The next pipeline on that branch pushes them again.
- **Expensive jobs wait on the cheap lints of their code.** `frontend-e2e-tests` and
  `app-frontend-image` need `frontend-lint` and `frontend-types`, and `app-backend-image` needs
  `rust-format` and `rust-clippy`. A lint failure then no longer pays for a browser run or a
  release build. Every other job keeps `needs: []` and starts at once.
- **Those `needs` are `optional: true`.** A lint job is not created when its files did not change,
  and a required `needs` on a missing job makes the whole pipeline invalid.

## Traps

Traps when adding a job:

- **`changes` paths and `pathspecs` use different syntaxes.** The first is a
  GitLab glob (`"**/*.md"`), the second a git pathspec (`*.md`). Update both together, or the
  job starts and finds no file, or never starts.
- **The file list is relative to the job's directory.** The `.bun` template moves to
  `frontend/`, so its jobs get paths relative to it, ready for `prettier` or `eslint`.
- **A command containing a colon followed by a space must be a block scalar** (`- >-`).
  Otherwise YAML reads it as a map and the job is invalid.
- **Docker jobs need a privileged runner** for the `docker:dind` service. A container they start
  listens on the `docker` host, not on `127.0.0.1`.
- **Images are pushed only from the default branch and tags.** On other branches they are built,
  then dropped. Container scanning runs only where images are pushed.

## Verification

```bash
./scripts/test_lint_cicd.sh    # every manifest against GitLab's CI schema
npx gitlab-ci-local --list     # the merged pipeline, includes and !reference resolved
```

The first needs `check-jsonschema` and validates each file alone; a file with a `spec:` header
is split and its two documents are checked separately. The second sees only files
known to git, so `git add` a new manifest first. `glab ci lint` checks the merged pipeline
against a real GitLab instance, with a token.
