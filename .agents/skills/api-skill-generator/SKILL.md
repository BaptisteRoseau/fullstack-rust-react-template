---
name: api-skill-generator
description: Use when a product exposes a REST API and no skill for it exists yet, or when navigating an OpenAPI manifest.
allowed-tools:
    - Read
    - Bash
---

# API Skill Generator

Turn an OpenAPI 3.x specification (URL or file) into an AI Agent Skill: a tree of
markdown files describing endpoints, schemas, and authentication that an agent can
navigate efficiently.

The GitHub repository of this tool is `https://github.com/BaptisteRoseau/openapi2skill`.

## Prerequisite

`openapi2skill` must be installed.
Consider it installed, but if the command is not found follow
[install.md](./install.md) instructions then resume your task. Do not read it otherwise.

## Naming and location

Generated skills must be named `api-<service>` and placed under
`.agents/skills/api-<service>` (e.g. `.agents/skills/api-stripe`,
`.agents/skills/api-keycloak`). Use a short, lowercase, kebab-case service name.

If running through Claude Code, prefer writing the skill under `.claude/skills/`
instead of `.agents/skills/`.

## Usage

Use this command to get `openapi2skill` usage.

```bash
openapi2skill --help
```

Example (spec fetched from a URL):

```bash
openapi2skill -f -o .agents/skills/api-grafana \
  https://raw.githubusercontent.com/example/grafana/main/openapi.json
```

Example (local spec file):

```bash
openapi2skill -f --server http://127.0.0.1:8080/ -o .agents/skills/api-keycloak ./specs/keycloak-openapi.json
```

### Overriding the server URL with `--server`

Specs often declare a server URL that does not match where the service actually
runs — for instance the spec says `https://api.example.com` but you reach the
service on `http://localhost:8080` or via a Docker container hostname. Override it
so the generated skill documents the URL you will actually call:

```bash
openapi2skill -f -o .agents/skills/api-<service> \
  --server http://localhost:8080 <PATH_OR_URL>
```

- If the URL has no scheme, `https://` is prepended automatically
  (`--server api.example.com` → `https://api.example.com`).
  Only prepend the scheme when using plain `http://`, for local development environement for example.
- `--server` can be passed multiple times to record several environments
  (e.g. local + staging):

```bash
openapi2skill -f -o .agents/skills/api-<service> \
  --server http://platform:8000 \
  --server https://staging.example.com \
  <PATH_OR_URL>
```

For a service running in a Docker container, use the hostname/port reachable from
where the agent runs (the published host port, or the container name on a shared
Docker network), not the spec's default.

## Finding the OpenAPI V3 Spec

If an OpenAPI specification file is not already pointed at
by the user (either URL or file), and you need to find it yourself,
read [find-openapi-spec.md](./find-openapi-spec.md)

## After generating

1. Confirm the skill was written: check that `.agents/skills/api-<service>/SKILL.md`
   exists and its frontmatter `name` is `api-<service>`.
2. Skim `SKILL.md` and `endpoints/index.md` to verify the spec was parsed as
   expected.
3. The new skill is then available for navigating that product's API.
4. Invoke the newly generated `api-<service>` skill and use it to make a real call
   against the API, to confirm it works end to end. The call does not need to
   succeed with data: reaching the API is enough — even a `401`/`403` from missing
   authentication proves the skill points at the right, reachable endpoint.
