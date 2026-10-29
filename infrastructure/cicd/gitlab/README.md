# GitLab CI

The GitLab CI pipeline. [gitlab-ci.yml](./gitlab-ci.yml) is its entry point, included by the
root [.gitlab-ci.yml](../../../.gitlab-ci.yml), and it includes every other file here.

The checks it runs and when they run are shared with every provider, see
[cicd/README.md](../README.md). This README covers how the GitLab manifests implement them.

## Directory

```txt
gitlab/
├── gitlab-ci.yml                 # stages, CI image tags, workflow rules, and the includes below
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

## CI images

Every job runs in an image of [infrastructure/docker](../../docker/README.md)`/ci_<name>`, so no
job installs a tool at run time:

| Image | Template | Holds |
| --- | --- | --- |
| `ci_rust` | `.rust` | rustfmt, clippy, llvm-tools, cargo subcommands, sccache, Docker CLI, buildx |
| `ci_bun` | `.bun` | Bun, git |
| `ci_playwright` | `.bun-playwright` | Playwright browsers, Bun, oxaria |
| `ci_tools` | `.tools` | the docs, shell, YAML, Dockerfile, Kubernetes and CI manifest linters |
| `ci_docker` | `.docker` | Docker CLI, bash, git |

A tool version lives in its image's Dockerfile, not in the job.

The `ci-<name>-image` jobs of the `ci_images` stage build them, and every job using an image
`needs` its build job with `optional: true`. They run only when an image directory changed, and
on every full run.

- **The tag is set by hand** in [gitlab-ci.yml](./gitlab-ci.yml), such as `ci_rust:1`. A job
  cannot pick its `image` from a value computed during the pipeline.
- **Bump the tag whenever the image directory changes.** The build job also pushes a tag holding
  the hash of the directory. When the image is already pushed but that hash tag is missing, the
  job fails and asks for a bump, so a changed Dockerfile never hides behind an old tag.
- **The CI images run as root**, unlike the release images: the job writes into the clone the
  runner made as root.

## Caches

- **Dependencies.** `CARGO_HOME` and Bun's install cache move into `.cache/` in the project
  directory, the only place GitLab caches. Jobs share the `cargo-*` and `bun-*` cache keys,
  computed from `Cargo.lock` and `frontend/bun.lock`. A new lock file starts a new cache, so
  dependencies it dropped do not pile up. A branch that changes a lock file has no cache for it
  and downloads its dependencies until it is merged.
- **Compiled Rust.** Jobs extending `.rust-sccache` compile through sccache. It uses the S3 bucket
  from the `SCCACHE_BUCKET`, `SCCACHE_ENDPOINT`, `SCCACHE_REGION` and `AWS_*` project variables,
  such as one on the stack's SeaweedFS. With no bucket set, it falls back to `.cache/sccache`,
  cached per job.
- **Docker layers.** Image builds read and write a registry cache, `$BUILD_CACHE:<image>`. They
  also build `tmp_health_checker` from that cache, which `.build-health-checker` shares with the
  `dockerfiles` job. `rust-generated-models` builds the database image from the cache of
  `app-postgres-image`.
- **Only the default branch writes.** The workflow sets `CACHE_POLICY` to `pull-push` there and
  `pull` everywhere else, and every cache and `--cache-to` follows it. A branch with new
  dependencies downloads them, and they are cached once the branch is merged.

Two instance settings make the caches reach every job:

- **A distributed cache.** Without `[runners.cache]` pointing at S3 in the runner's
  `config.toml`, each runner keeps its own cache, and a job that lands on another runner starts
  empty.
- **Shared caches across branches.** Branches only read the default branch's cache when the
  project setting "Use separate caches for protected branches" is disabled. Otherwise every
  branch pipeline starts empty, without any error.

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
  `rust-format` and `rust-check`. A lint failure then no longer pays for a browser run or a
  release build. Every other job keeps `needs: []` and starts at once.
- **Those `needs` are `optional: true`.** A lint job is not created when its files did not change,
  and a required `needs` on a missing job makes the whole pipeline invalid.

## Compiling Rust once per mode

A full pipeline compiles the workspace three times, once per mode:

- **`rust-check`** runs clippy, rustdoc and the check without default features. All three
  compile in check mode, so they share the compiled dependencies.
- **`rust-tests`** runs every test once, doctests and container tests included, in the build
  cargo-llvm-cov instruments, and reports coverage. The OpenAPI document reuses that build:
  `openapi-sdk` checks the SDK against it, so it compiles nothing.
- **`app-backend-image`** builds the release binary. cargo-chef keeps the dependencies in their
  own layer, so it only rebuilds them when `Cargo.toml` or `Cargo.lock` change.

`openapi-sdk` reuses the rules of `rust-tests`, and `rust-tests` also runs when only the SDK
changed. Keep them that way: the needs of `openapi-sdk` on `rust-tests` is not optional, so a
pipeline where only the first exists is invalid.

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
- **A template's `needs` is replaced, not merged.** A job that sets its own `needs`, such as
  `app-backend-image`, must list its image's build job again, or it can start before its image
  is pushed. The changed-files template sets no `needs` for that reason.
- **`frontend/dist` replaces a Dockerfile stage.** `app-frontend-image` passes the dist of
  `frontend-build` as the `site` stage of its Dockerfile. When that job did not run, the stage is
  built from the sources instead.
- **Images are pushed only from the default branch and tags.** On other branches they are built,
  then dropped. Container scanning runs only where images are pushed.
- **A tag retags the image of its commit.** When the default branch already pushed
  `<image>:<short sha>`, the tag pipeline adds the tag to it without building. A tag pushed
  before that pipeline finished, or on a commit that never was on the default branch, builds the
  image instead.

## Verification

```bash
./scripts/test_lint_cicd.sh    # every manifest against GitLab's CI schema
npx gitlab-ci-local --list     # the merged pipeline, includes and !reference resolved
```

The first needs `check-jsonschema` and validates each file alone; a file with a `spec:` header
is split and its two documents are checked separately. The second sees only files
known to git, so `git add` a new manifest first. `glab ci lint` checks the merged pipeline
against a real GitLab instance, with a token.
