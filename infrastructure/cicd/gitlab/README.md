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
│   └── <category>.gitlab-ci.yml  # changes rules shared by several jobs: .frontend-changed
├── templates/
│   ├── <tool>.gitlab-ci.yml      # hidden jobs shared by several jobs, such as .rust or .buildkit
│   └── inputs/
│       └── <name>.gitlab-ci.yml  # templates with spec:inputs, included by the jobs using them
└── jobs/
    └── <category>.gitlab-ci.yml  # the jobs that run, one file per category of cicd/README.md
```

**Every manifest ends in `.gitlab-ci.yml`.** The includes and `test_lint_cicd.sh` select files by
that suffix, so a manifest named `<name>.yml` is silently left out of the pipeline and of the
lint.

- **`templates/`** holds hidden jobs, whose names start with a dot. Each one implements how a
  kind of job runs, for example how to start a BuildKit daemon.
- **`templates/inputs/`** holds templates that take [`spec:inputs`](#templates-with-inputs). The
  entry point's `templates/*.gitlab-ci.yml` wildcard does not reach this folder, on purpose: a
  wildcard include passes no inputs, so the pipeline would fail on the missing ones.
- **`jobs/`** holds the jobs that actually run. Each one extends a template and names what it
  applies to, for example `app-backend-image`.
- **`rules/` and `templates/` hold only what two jobs or more share.** A rule or a template used
  by a single job lives in that job, where its reader looks for it.

## Rules

Templates and jobs can both define `rules`. A template sets the default trigger for its kind of
job. A job that needs another trigger replaces it, because `extends` **replaces** `rules`
instead of merging them.

A shared rule is a hidden key named after what changed, `.<name>-changed`, holding a `rules`
list. [changes_rule.gitlab-ci.yml](./templates/inputs/changes_rule.gitlab-ci.yml) generates it
from a `name` and a `paths` list, so `compare_to` is written once. A job lists shared rules next
to the paths only it needs:

```yaml
app-keycloak-image:
  extends: .app-keycloak-image
  rules:
    - !reference [.should-run-all-jobs, rules]
    - !reference [.tmp-health-checker-image-changed, rules]
    - changes:
        paths:
          - infrastructure/docker/app_keycloak/**/*
        compare_to: refs/heads/$CI_DEFAULT_BRANCH
```

A job runs when any one of its rules matches, so splitting a path list across several rules does
not change when it runs.

## Changed files

The workflow in [gitlab-ci.yml](./gitlab-ci.yml) sets `SHOULD_RUN_ALL_JOBS` to `true` wherever
every check must run on every file. `.should-run-all-jobs` comes first in every job, so it then
matches before any `changes` rule.

`changes` rules compare to the default branch (`compare_to: refs/heads/$CI_DEFAULT_BRANCH`).
Variables in `compare_to` need GitLab 17.2 or later. On an older instance they are not expanded,
and the jobs no longer start on a branch.

A job that lints changed files only uses
[lint_changed_files.gitlab-ci.yml](./templates/inputs/lint_changed_files.gitlab-ci.yml), see
below. One glob both starts the job and lists its files, through
[list_changed_files.sh](../../../scripts/list_changed_files.sh), which reads GitLab's glob syntax.

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
      paths: "{**/*.sh,scripts/git_hooks/pre-push}"
      command: shellcheck

shellcheck:
  extends:
    - .tools
    - .shellcheck-changed-files
```

| Template | Generates | Inputs |
| --- | --- | --- |
| [lint_changed_files](./templates/inputs/lint_changed_files.gitlab-ci.yml) | `.<job>-changed-files`, with its rules | `job`, `paths`, `exclude`, `all_on_delete`, `command` |
| [docker_image](./templates/inputs/docker_image.gitlab-ci.yml) | `.<job>`, and the visible `<job>-scan` | `job`, `image`, `dockerfile`, `prebuilt_stage`, `prebuilt_directory`, `needs` |
| [changes_rule](./templates/inputs/changes_rule.gitlab-ci.yml) | `.<name>-changed` | `name`, `paths` |

- **`paths` of `lint_changed_files` is one GitLab glob.** Braces list several:
  `"**/*.{yml,yaml}"`. `exclude` leaves files out of the lint, but they still start the job,
  since `rules:changes` cannot exclude. `all_on_delete` lints every matching file when the branch
  deleted or renamed a file, for a check that such a file can break, such as a markdown link.
- **`needs` of `docker_image` lists the jobs to wait for besides `ci-docker-image`.** The template
  adds them to its own, since a job's `needs` would replace the template's instead of merging.
- **An array input goes in as a JSON-like list**, such as `paths: ["frontend/**/*"]`. Never put a
  `!reference` inside an input: GitLab does not resolve it there.

## CI images

Every job runs in an image of [infrastructure/docker](../../docker/README.md)`/ci_<name>`, so no
job installs a tool at run time:

| Image | Template | Holds |
| --- | --- | --- |
| `ci_rust` | `.rust` | rustfmt, clippy, llvm-tools, cargo subcommands, sccache, Docker CLI, buildx |
| `ci_bun` | `.bun` | Bun, git |
| `ci_playwright` | `.bun-playwright` | Playwright browsers, Bun, oxaria |
| `ci_tools` | `.tools` | the docs, shell, YAML, Dockerfile, Kubernetes and CI manifest linters |
| `ci_docker` | `.docker-cli` | Docker CLI, bash, git |

A tool version lives in its image's Dockerfile, not in the job.

The `ci-<name>-image` jobs of the `ci_images` stage build them, and every job using an image
`needs` its build job with `optional: true`. They run only when an image directory changed, and
on every full run. They stay five jobs rather than one `parallel:matrix` job, because GitLab
refuses `optional` on a `needs` of a matrix job.

- **The tag is the hash of the image directory**, such as `ci_rust:7d30afb0266a`, written in
  [gitlab-ci.yml](./gitlab-ci.yml). A job cannot pick its `image` from a value computed during
  the pipeline, so the hash is committed.
- **Run `./scripts/update_ci_image_tags.sh` after changing an image directory**, and commit the
  entry point with it. The build job hashes the directory the same way and fails on a stale tag,
  and so does `test_lint_cicd.sh`, which the `pre-push` hook runs.
- **Equal files give equal tags.** Two branches changing an image differently push different
  tags, so they never overwrite each other's image.
- **The hash covers the files git tracks.** `git add` a new file of the image directory before
  running the script, or the job, which hashes every file of its clone, disagrees with it.
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
- **Docker layers.** Image builds, in
  [build_release_image.sh](../../../scripts/build_release_image.sh), read and write a registry
  cache, `$BUILD_CACHE:<image>`. They also build `tmp_health_checker` from that cache, which the
  `dockerfiles` job shares. `rust-generated-models` builds the database image from the cache of
  `app-postgres-image`.
- **One cache key, one set of paths.** `.rust` and `.rust-sccache` push the same `cargo-*` key,
  so they share its entry through a YAML anchor. Two lists of paths under one key would let the
  last job to upload drop the others' paths.
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

## Runners

Jobs start containers or build images in two ways, and each needs its own runner setting:

| Template | Jobs | Runs | Runner |
| --- | --- | --- | --- |
| `.buildkit` | `ci-<name>-image`, `app-<service>-image` | a rootless BuildKit service | any |
| `.docker-daemon` | `rust-tests`, `rust-generated-models`, `dockerfiles` | a `docker:dind` service | tagged `privileged` |

- **Image builds need no privileged runner.** Rootless BuildKit only needs the seccomp and
  AppArmor profiles of its service to be unconfined, so it can create a user namespace:

  ```toml
  [runners.docker]
    services_security_opt = ["seccomp:unconfined", "apparmor:unconfined"]
  ```

- **Containers need a privileged runner.** The integration tests and the generated models start
  containers, and `docker build --check` needs `tmp_health_checker` loaded in a daemon. Only a
  runner with `privileged = true` and the `privileged` tag picks those jobs up. Without one, they
  stay pending.

## Cancelled and waiting jobs

- **A new commit cancels the interruptible jobs of its branch's older pipelines.** The workflow
  sets `auto_cancel: on_new_commit: interruptible`, and `default:` makes every job
  interruptible. A job set to `interruptible: false` keeps running and the rest are cancelled.
  The project setting "Auto-cancel redundant pipelines" must stay enabled: the workflow only
  tunes it.
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

## Security scans and dependency updates

[security.gitlab-ci.yml](./jobs/security.gitlab-ci.yml) includes GitLab's scanners:

| Template | Jobs | Scans |
| --- | --- | --- |
| `Secret-Detection` | `secret_detection` | secrets in the commits |
| `SAST` | `semgrep-sast` and the other analyzers | the sources |
| `SAST-IaC` | `kics-iac-sast` | Dockerfiles, Compose and Kubernetes manifests |
| `Dependency-Scanning.v2` | `dependency-scanning` | `Cargo.lock` and `bun.lock` |
| `Container-Scanning` | `<job>-scan`, one per release image | the pushed images |

- **Dependency scanning needs GitLab 19.0 or later**, where its `v2` template is generally
  available. On an older instance, the include fails and no pipeline is created.
- **`AST_ENABLE_MR_PIPELINES` is `"false"`.** The templates otherwise leave a branch with an
  open merge request to a merge request pipeline, which the workflow never creates, and the scans
  would not run at all. Dependency scanning does so even when the variable is unset.
- **The scanners start at once.** `needs: []` on their bases keeps a failing lint from leaving the
  commit unscanned.

`dependency-updates` in [fullstack.gitlab-ci.yml](./jobs/fullstack.gitlab-ci.yml) runs Renovate
on the clone, as configured by
[configs/renovate/config.json](../../configs/renovate/config.json). It looks up a
newer version of every crate, npm package and Docker image, including the images of these
manifests, and lists them in its log and in `renovate-report.json`. It opens no merge request and
needs no token. A `GITHUB_COM_TOKEN` project variable lifts GitHub's rate limit on its lookups.

- **An image given through a variable is invisible to Renovate**, such as `docker:$DOCKER_VERSION`.
  The config reads `DOCKER_VERSION` with a custom manager: give any new version variable one.
- **The config is passed as `RENOVATE_CONFIG_FILE`**, not found at the repository root. A
  Renovate bot installed on the project would not read it.

## Traps

Traps when adding a job:

- **The file list is relative to the job's directory.** The `.bun` template moves to
  `frontend/`, so its jobs get paths relative to it, ready for `prettier` or `eslint`.
- **A command containing a colon followed by a space must be a block scalar** (`- >-`).
  Otherwise YAML reads it as a map and the job is invalid.
- **A container started by a `.docker-daemon` job listens on the `docker` host**, not on
  `127.0.0.1`.
- **BuildKit has no image store.** An image a `.buildkit` job builds cannot be referenced by a
  later build in the same job. Pass it as a `--build-context`, as `build_release_image.sh` does
  with `tmp_health_checker`.
- **A template's `needs`, `cache` and `rules` are replaced, not merged.** A job that sets its own
  `needs` must list its image's build job again, or it can start before its image is pushed.
  `!reference` cannot help: GitLab only combines lists of them in `rules`, `script` and `stages`.
  Pass extra needs as an input instead, as `docker_image` does. The changed-files template sets
  no `needs` for that reason.
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
./scripts/test_lint_cicd.sh      # every manifest against GitLab's CI schema, and the CI image tags
npx gitlab-ci-local --preview    # the merged pipeline, includes, inputs and !reference resolved
```

The first needs `check-jsonschema` and validates each file alone; a file with a `spec:` header
is split and its two documents are checked separately. The second sees only files
known to git, so `git add` a new manifest first. `glab ci lint` checks the merged pipeline
against a real GitLab instance, with a token.
