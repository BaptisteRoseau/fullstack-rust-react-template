# CICD

This directory contains manifests for GitHub Actions and GitLab CI.

See [github](./github) and [gitlab](./gitlab) respectively.

## Conventions

Most CI/CD jobs especially linters, formatters, tests and builds are written in scripts to be runnable on a developpper's machine.
The CI/CD jobs then simply call the script after setting-up the environment (dependency cache, environment variables..).
