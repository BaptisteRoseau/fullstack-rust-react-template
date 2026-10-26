# GitLab CI/CD improvement plan

Step 3 of the GitLab CI/CD work: fixes and optimizations of the pipeline in
[infrastructure/cicd/gitlab](../infrastructure/cicd/gitlab/README.md). Every item below is still
to do; an item is removed from this file once it is implemented.

Items are in the order they should be done: later ones build on earlier ones. Each one says what
changes, why, and how to know it is done.

## Current state

- **Written, never run.** The pipeline is committed and its README describes how it works. It
  passes the schema check of `scripts/test_lint_cicd.sh` and resolves with `gitlab-ci-local`,
  but has not run on a GitLab instance yet.
- **Known red jobs.** Item 0 lists the jobs expected to fail on the first run.
- **Next.** Item 3, caching. Item 0 needs a GitLab runner to confirm, and item 1 can follow
  item 3, since caching does not depend on the CI images.

## 0. Make the existing jobs pass

The pipeline has never run on a GitLab instance. These failures are known before it does:

- **Integration tests connect to `127.0.0.1`.** The four `crates/*/tests/backends/*.rs` fixtures
  (`cache`, `authenticator`, `storage`, `database`) build their URL from `127.0.0.1`. Under
  Docker-in-Docker the containers listen on the `docker` host, so every integration test fails.
  Use the host testcontainers reports (`container.get_host()`) instead.
- **New gating linters fail on the current code**: `typos` reports 5 issues, `hadolint` 5 and
  `shellcheck` 37. Fix the code, or justify each exception in the linter's config.
- **Never run locally**: `rust-doc` with `-D warnings`, `rust-generated-models` and `nix`. Run
  them once and fix what they report.

Done when a pipeline on the default branch, where every job runs, is green.

## 1. Build the CI images once

Almost every job installs its tools before it starts: `apt-get install git` in `.bun`,
`npm install --global`, `pip install`, cargo-binstall, `curl | bash` for Bun, and
`rustup component add`. That costs time on every job, depends on the network, and spreads the
tool versions across many files.

- Add `infrastructure/docker/ci_<name>/Dockerfile` images, such as `ci_rust` (toolchain
  components, cargo subcommands, Docker CLI, Bun), `ci_bun` (git) and `ci_tools` (markdownlint,
  markdown-link-check, cspell, typos, yamllint, check-jsonschema, shellcheck, hadolint). The
  `ci_*` prefix is already reserved by [docker/README.md](../infrastructure/docker/README.md).
- Build them in a first job, tagged with a hash of their Dockerfile, and push them to the project
  registry. Rebuild only when that hash is not in the registry yet.
- Point the templates at `$CI_REGISTRY_IMAGE/ci_<name>:<hash>` and drop the install steps.

This is the approach of freedesktop's ci-templates, used by Mesa and GNOME.

Done when no job installs a tool at run time.

## 2. Compile the Rust workspace fewer times

`rust-clippy`, `rust-no-default-features`, `rust-doc`, `rust-unit-tests`,
`rust-integration-tests` and `rust-coverage` each build the whole workspace from scratch.

- Make `rust-coverage` the integration test job: it runs every test once, produces the report,
  and becomes gating. Remove `rust-integration-tests`.
- Run `rust-clippy` and `rust-doc` in one job, so they share `target/`.

Done when a full pipeline compiles the workspace at most four times.

## 3. Cache dependencies and builds

- **Cargo registry.** Move `CARGO_HOME` into the project directory, because GitLab only caches
  paths inside it. Cache `registry/` and `git/`, keyed on `Cargo.lock`.
- **Cache policy.** `policy: pull` on branches, `pull-push` on the default branch, with
  `fallback_keys` pointing at the default branch's cache.
- **Compiled output.** Prefer sccache with an S3 backend over caching `target/`. GNOME's librsvg
  measured about 400 MB of `target/` cache for little gain. The stack already runs SeaweedFS,
  which speaks S3.
- **Bun.** Cache the package store (`BUN_INSTALL_CACHE_DIR` inside the project), not
  `node_modules`, keyed on `frontend/bun.lock`.

Done when a second pipeline on the same `Cargo.lock` and `bun.lock` downloads no dependency.

## 4. Cache Docker layers and build the health checker once

Docker-in-Docker starts empty, so every image build starts without a layer cache.

- Build with `docker buildx build --cache-from type=registry --cache-to type=registry`.
- Build `tmp_health_checker` once, push it, and hand it to the other builds with
  `--build-context localhost/tmp_health_checker=docker-image://<pushed image>`. Today the backend,
  frontend, keycloak and homepage jobs each rebuild it.
- Consider rootless BuildKit (`moby/buildkit:rootless`), which needs no privileged runner.

Done when rebuilding an unchanged image takes under a minute.

## 5. Show results in GitLab

- **Tests.** Switch Rust tests to `cargo nextest` and publish its JUnit output as
  `artifacts: reports: junit`. Keep `cargo test --doc`, because nextest does not run doctests.
  Add the JUnit reporters of Vitest and Playwright. This closes the TODO "Convert tests results
  as junit tests".
- **Lint findings.** Publish clippy and ESLint findings as `reports: codequality`, as
  `frontend-accessibility` already does with oxaria.
- **Coverage.** Add a `coverage:` regex to both coverage jobs, so the percentage shows on the
  pipeline.

Done when a failing test shows in the pipeline's Tests tab.

## 6. Reliability

- **Retry infrastructure failures only.** In `default:`, set `retry: max: 2` with
  `when: [runner_system_failure, stuck_or_timeout_failure, scheduler_failure, api_failure]`.
  Never retry a script failure: it would hide a flaky test.
- **External links.** The link check is gating and checks external URLs, so a site down blocks a
  push. Add `retryOn429`, `retryCount` and `aliveStatusCodes` to `.markdown-link-check.json`.
- **Timeouts.** Give each job a `timeout`, so a hung e2e test does not hold a runner for the
  default hour.

## 7. Supply chain

- **Renovate.** The manifests pin about 30 versions. Run Renovate on the GitLab instance, with
  custom regex managers for the `*_VERSION` variables and the image tags, so they are bumped
  through merge requests instead of going stale.
- **Digests.** Pin images by digest as well as by tag.
- **No `curl | bash`.** Item 1 removes the cargo-binstall and Bun installers from the jobs.

## 8. Structure

- **No parent-child pipelines.** At about 50 jobs they add more complexity than they save.

## 9. Later: release and deploy

On tags: images tagged with the version, an SBOM from `syft`, and a release created with the
`release:` keyword. A deploy job then uses `environment:` and `resource_group:`, so two deploys
never run at once.

## Known limitation

Merge request pipelines are not created, so a branch is tested alone, not merged with the latest
default branch. Merged results pipelines and merge trains would cover that, but they are Premium
features and the instance is free self-managed. The full run on the default branch is the safety
net: keep it required and watched.

## After this plan

Write a skill for adding and changing GitLab CI jobs, from the conventions in
[infrastructure/cicd/gitlab/README.md](../infrastructure/cicd/gitlab/README.md).

## Sources

- [GitLab: optimize CI/CD configuration files](https://docs.gitlab.com/ci/yaml/yaml_optimization/)
- [GitLab: CI configuration performance](https://docs.gitlab.com/development/pipelines/performance)
- [GitLab: build Docker images with BuildKit](https://docs.gitlab.com/ci/docker/using_buildkit/)
- [GNOME librsvg: speed up CI pipeline](https://gitlab.gnome.org/GNOME/librsvg/-/merge_requests/398)
- [Faster CI builds for Rust with builder images and sccache](https://vadosware.io/post/faster-ci-builds-for-rust-with-builder-images-and-sccache/)
- [Depot: guide to faster Rust builds in CI](https://depot.dev/blog/guide-to-faster-rust-builds-in-ci)
