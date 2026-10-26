# MCP

The Model Context Protocol layer. Exposes part of the backend as **tools** that assistants
(Claude Code, Claude Desktop, an agent SDK) can call, over one HTTP endpoint that
[api](../api) mounts.

It sits next to `api`, not below it: both turn [app_core](../app_core) and the service crates
into a protocol. See the layer rule in [crates/README.md](../README.md).

```txt
                       ┌─ api  (HTTP + OpenAPI) ─┐
   MCP client ── HTTP ─┤                         ├─ app_core ─ database
                       └─ mcp  (MCP tools)   ────┘
```

## Public surface

- [`McpState`](src/state.rs) — the services the tools may reach. `api` builds it from its
  `AppState`. It holds `Arc<RwLock<dyn Trait>>`, so a tool never knows which backend it uses.
- [`mcp_routes`](src/routes.rs) — builds the `Router` holding the endpoint. It is generic over
  the caller's state, so it can be merged into `api`'s router before that router gets its state.

Everything else stays private: the tools, the `ServerHandler`, `McpError` and `rmcp`. This way,
everything an MCP client can reach is listed in one file, [server.rs](src/server.rs).

`McpError` breaks the error convention of [crates/README.md](../README.md) on purpose. It never
leaves the crate, because MCP reports a tool failure inside the protocol response, not as a Rust
error.

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

## Transport

The endpoint speaks [Streamable
HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports), served by
`rmcp`'s `StreamableHttpService`. There is no stdio transport: stdio lets a client start a
server as a child process, and this backend is already running and listening.

Sessions are off (`legacy_session_mode: false`), because the tools keep no state between calls.
Without sessions, the endpoint answers with a plain `application/json` body instead of an open
event stream. This matters: the endpoint sits behind the API's `TimeoutLayer`, which would cut a
long-lived stream.

## Tools

A tool is the MCP version of an `api` handler: it takes its arguments, calls `app_core`, and
serialises the result.

Each group of tools is an `impl McpServer` block with `#[tool_router]`. The groups are added
together in the `#[tool_handler]` attribute in [server.rs](src/server.rs), the same role as
`api`'s `routes/router.rs`. A tool left out of that sum compiles fine but is never offered to
clients.

| Tool | What it does |
| --- | --- |
| `ping` | Answers `pong`. The counterpart of `GET /api/ping`. |
| `get_user` | Reads one user's profile by UUID, through `app_core::user::read_profile`. |

## Authentication

The tools know nothing about identity. `api` wraps the whole endpoint in the same `UserToken`
check as its other authenticated endpoints ([routes/router.rs](../api/src/routes/router.rs)).
An anonymous caller is refused before any tool runs.

A client logs in like any other API caller: an API key in the `Authorization` header, or the
`access_token` cookie set by the auth BFF.

**Not yet done:** every logged-in caller can use every tool. Per-tool permission checks would go
through [rbac](../rbac), which is not wired in yet.

`--mcp-allowed-hosts` is the `Host` header allow list. The MCP specification requires it on HTTP
transports to block DNS-rebinding attacks.

**WARNING**: it defaults to the loopback names. A deployment answering on a real domain must add
that domain, or every request is refused.

## Configuration

`McpConfig` in [config](../config), built only when the endpoint is enabled:

| Flag / env | Default | Meaning |
| --- | --- | --- |
| `--mcp-path` | `/mcp` | Path of the endpoint, at the server root, not under `/api` |
| `--mcp-allowed-hosts` | `localhost,127.0.0.1,::1` | Accepted `Host` headers; empty disables the check |
| `--mcp-server-name` | `backend` | Server name sent to clients during the handshake |
| `--mcp-server-instructions` | see [defaults.rs](../config/src/defaults.rs) | What the model is told the server is for |
| `--no-mcp-json-response` | `false` | Stream the result instead of answering with one JSON body |
| `--no-mcp` | `false` | Do not mount the endpoint at all |

## Running

```bash
source .env && cargo run -p backend
```

```bash
# List the tools. The transport requires both Accept types.
curl -sS http://localhost:8080/mcp \
  -H 'Authorization: <api key>' \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

To connect a client to the same URL, for example Claude Code:

```bash
claude mcp add --transport http backend http://localhost:8080/mcp \
  --header "Authorization: <api key>"
```

## Rules

- Never import `api` from this crate.
- Only `McpState` and `mcp_routes` are public; every other item stays crate-private.
- No business logic in a tool: it calls `app_core` and serialises the result.
- Every tool group must be added to the `#[tool_handler]` sum in `server.rs`.
- No stdio transport.
- Keep sessions off, so responses stay plain JSON and survive the API's `TimeoutLayer`.

## Skills

- [backend-add-mcp-tool](../../.claude/skills/backend-add-mcp-tool/SKILL.md)
- [backend-config-entry](../../.claude/skills/backend-config-entry/SKILL.md)
