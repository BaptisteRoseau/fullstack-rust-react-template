# MCP

The Model Context Protocol layer. Exposes part of the backend to MCP clients — Claude Code,
Claude Desktop, an agent SDK — as **tools** an assistant can call, over a single HTTP
endpoint that [api](../api) mounts.

It is a sibling of `api`, not a layer below it: both turn [app_core](../app_core) and the
service crates into a protocol. See [crates/README.md](../README.md) for the layer rule —
`mcp` may use `app_core` and `database`, and must never import `api`.

```txt
                       ┌─ api  (HTTP + OpenAPI) ─┐
   MCP client ── HTTP ─┤                         ├─ app_core ─ database
                       └─ mcp  (MCP tools)   ────┘
```

## Public surface

Two items, and nothing else:

- `McpState` ([state.rs](src/state.rs)) — the services the tools may reach. `api` builds it
  from its own `AppState`; it holds `Arc<RwLock<dyn Trait>>`, so a tool never knows which
  backend is behind a trait.
- `routes::mcp_routes` ([routes.rs](src/routes.rs)) — builds the `Router` carrying the
  endpoint, generic over the caller's state so it merges into `api`'s router before that
  router is given its state.

The tools, the `ServerHandler`, `McpError` and `rmcp` itself stay private. A caller can
mount the endpoint and configure it, and can do nothing else with this crate — which is the
point: everything reachable from an MCP client should be visible in one file
([server.rs](src/server.rs)), not scattered across whatever `api` happens to call.

`McpError` being crate-private is a deliberate deviation from the error convention in
[crates/README.md](../README.md): it never crosses the boundary, because MCP carries a
tool's failure inside the protocol response rather than in a Rust error.

## Transport

**HTTP only.** The endpoint speaks [Streamable
HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports), the
current MCP transport, served by `rmcp`'s `StreamableHttpService`. There is no stdio
transport and there should not be one: stdio exists so a client can spawn a server as a
child process, and this server is a long-running backend that is already listening.

Sessions are off (`legacy_session_mode: false`). The tools hold no per-connection state, and
a stateless endpoint answers a plain `application/json` body rather than an open event
stream — which matters, because the endpoint sits behind the API's `TimeoutLayer` and a
long-lived stream would be cut by it.

## Directory

```txt
mcp/
├── src/
│   ├── state.rs     # McpState: what the tools may reach
│   ├── server.rs    # McpServer, the handshake info, and the list of tool routers
│   ├── routes.rs    # mcp_routes: the one public entry point
│   ├── tools/       # the tools, one directory per group — see tools/README.md
│   └── error/       # McpError and its conversion to a tool result
└── Cargo.toml
```

See [tools/README.md](src/tools/README.md) for that directory.

## Tools

A tool is the MCP equivalent of an `api` handler: it takes its arguments, calls `app_core`,
and serialises the result. Business logic never lives here.

Each group is an inherent `impl McpServer` block carrying `#[tool_router]`, exactly as each
`api` endpoint group is a module of `#[utoipa::path]` handlers. The routers are summed in
the `#[tool_handler]` attribute in [server.rs](src/server.rs), which is the counterpart of
`api`'s `routes/router.rs`: **a tool that is written but not summed there does not exist.**

| Tool | What it does |
| --- | --- |
| `ping` | Answers `pong`. The counterpart of `GET /api/ping`. |
| `get_user` | Reads one user's profile by UUID, through `app_core::user::read_profile`. |

## Authentication

The tools hold no notion of identity. `api` wraps the whole endpoint in the same `UserToken`
check its authenticated endpoints use ([routes/router.rs](../api/src/routes/router.rs)), so
an anonymous caller is refused before any tool runs, and a tool never has to re-check.

A client therefore authenticates the way any other API caller does: an API key in the
`Authorization` header, or the `access_token` cookie set by the auth BFF.

> This is coarse: every authenticated caller reaches every tool. Per-tool permission checks
> would go through [rbac](../rbac), which is not wired in yet.

The `Host` allow list (`--mcp-allowed-hosts`) is the DNS-rebinding guard the MCP
specification requires of HTTP transports. It defaults to the loopback names, so **a
deployment answering on a real domain must add it** or every request is refused.

## Configuration

`McpConfig` in [config](../config), built only when the endpoint is enabled:

| Flag / env | Default | Meaning |
| --- | --- | --- |
| `--mcp-path` | `/mcp` | Path of the endpoint, at the server root, not under `/api` |
| `--mcp-allowed-hosts` | `localhost,127.0.0.1,::1` | Accepted `Host` headers; empty disables the check |
| `--no-mcp-json-response` | `false` | Stream the result instead of answering with one JSON body |
| `--no-mcp` | `false` | Do not mount the endpoint at all |

## Trying it

```bash
source .env && cargo run -p backend
```

```bash
# List the tools. Both Accept types are required by the transport.
curl -sS http://localhost:8080/mcp \
  -H 'Authorization: <api key>' \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

Point a client at the same URL — for Claude Code:

```bash
claude mcp add --transport http backend http://localhost:8080/mcp \
  --header "Authorization: <api key>"
```

## Skills

- [backend-add-mcp-tool](../../.claude/skills/backend-add-mcp-tool/SKILL.md)
- [backend-config-entry](../../.claude/skills/backend-config-entry/SKILL.md)
