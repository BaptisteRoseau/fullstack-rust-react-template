# CI/CD

Pipeline manifests for GitLab CI and GitHub Actions. Each provider's standard entry point in the
repository root does one thing: include the entry point from this directory.

| Provider | Root entry point | Included from |
| --- | --- | --- |
| GitLab CI | [.gitlab-ci.yml](../../.gitlab-ci.yml) | [gitlab](./gitlab/README.md) |
| GitHub Actions | **Not implemented.** | [github](./github) |

## Directory

```txt
cicd/
├── gitlab/   # GitLab CI manifests — see gitlab/README.md
└── github/   # GitHub Actions workflows, empty for now
```

## Conventions

- **A job calls a script.** Linters, tests and builds live in [scripts](../../scripts/README.md)
  so they run the same on a developer's machine. A job only sets up the environment, then calls
  the script. A script that several jobs need with different triggers takes a section argument,
  such as `test_lint.sh backend`.
- **Pin every image and tool version.** A `latest` tag changes under a pipeline that did not,
  and the failure then has no matching commit.
- **Both providers run the same checks with the same triggers**, listed below, so a project
  hosted on either one gets the same gate.

## When a pipeline and a check run

There is **one pipeline per pushed commit**. Merge request (or pull request) pipelines are not
created, because they would run the same commit a second time.

Every check runs on every file on the default branch, on tags, on schedules, and when a CI
manifest changed. On other branches, a check runs only when the files it covers changed since
the branch left the default branch.

Checks on files that do not depend on each other, such as markdown or shell scripts, also look
only at the changed files, listed by [list_changed_files.sh](../../scripts/list_changed_files.sh),
and pass when none match. A deleted or renamed file can break a file that did not change, such
as a markdown link to it: those checks then look at every file.

## Checks

A non-gating check reports but never fails the pipeline.

| Category | Checks | Gating |
| --- | --- | --- |
| docs | markdownlint, markdown links including external URLs, typos | yes |
| docs | cspell | no |
| rust | rustfmt, clippy, build without default features, rustdoc, licenses | yes |
| rust | unit tests, integration tests, coverage report, generated database models | yes |
| rust | advisories, unused dependencies | no |
| frontend | lint, types, prettier, i18n catalogs, licenses | yes |
| frontend | unit tests, coverage report, e2e tests, production build, Storybook build | yes |
| frontend | advisories, accessibility | no |
| fullstack | committed API SDK matches the router | yes |
| fullstack | newer versions of the crates, npm packages and Docker images | no |
| infrastructure | Dockerfiles, hadolint, Compose, Kubernetes, Nix, shellcheck, CI manifests | yes |
| infrastructure | yamllint | no |
| security | secret detection, SAST, IaC scanning, dependency scanning, container scanning | no |
| build | one release image per service in [docker](../docker/README.md) | yes |
