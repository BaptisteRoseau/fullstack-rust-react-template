---
name: backend-add-mcp-tool
description: Use when adding, changing or removing an MCP tool exposed to assistants by the Rust backend (crates/mcp).
---

# Add a backend MCP tool

The `mcp` crate handles the Model Context Protocol only: take the arguments, call
`app_core`, serialise the result. **Business logic goes in
[app_core](../../../crates/app_core), never in a tool.** If you are writing domain rules
inside a tool, move them first.

Conventions for these files are described in
[tools/README.md](../../../crates/mcp/src/tools/README.md). Read
[crates/mcp/README.md](../../../crates/mcp/README.md) first if you have not.

**Before writing a tool, decide whether it should exist.** Everything here is reachable by
an assistant, and through it by whoever is talking to that assistant. A tool that returns
more than the caller asked for is a leak, not a feature.

## 1. Create the directory

One directory per tool group, under `crates/mcp/src/tools/<name>/`. Group by the domain
concept the tools act on, mirroring the `api` endpoint groups.

Copy the three templates in [assets/](./assets) into it, then rename the types:

```bash
cp .claude/skills/backend-add-mcp-tool/assets/{mod.rs,models.rs,tools.rs} \
   crates/mcp/src/tools/<name>/
```

A group whose tools take no arguments and return a plain `String` needs no `models.rs` —
delete it and drop the `mod models;` line.

## 2. Fill in `models.rs`

One struct for the parameters, one for the result, named `<Tool>Params` and `<Tool>Result`.

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Arguments of the `get_widget` tool.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetParams {
    /// Identifier of the widget to read, as a UUID.
    pub widget_id: Uuid,
}

/// A widget, as returned by the `get_widget` tool.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetResult {
    pub id: Uuid,
    pub name: String,
}
```

Rules:

- All structs derive `Debug`, `JsonSchema` and `#[serde(rename_all = "camelCase")]`.
- Parameter structs also derive `Deserialize`; result structs also derive `Serialize`.
- `JsonSchema` generates the tool's `inputSchema`, so **each field's doc comment is read by
  the model** deciding how to call the tool. Write them for that reader, not for yourself.
- Add `From<DomainType>` conversions here, so the tool stays free of mapping code.
- Omit any field the caller did not ask for. Permissions, hashes and internal ids do not
  belong in a result.

## 3. Fill in `tools.rs`

The template shows one tool. Keep to its shape:

- The block is an inherent `impl McpServer` carrying
  `#[tool_router(router = <name>_tool_router, vis = "pub(crate)")]`. Rust allows an inherent
  impl to be split across modules; that is what lets each group own its file.
- Annotate every tool with `#[tool(description = "...")]`. That text and the doc comment
  are what an assistant reads before choosing the tool: say what it returns and when it
  fails.
- Take arguments as `Parameters<T>`, destructured in the signature.
- Return `Result<CallToolResult, ErrorData>` and delegate to a private method returning
  `Result<CallToolResult, McpError>`, so the body can use `?`.
- Build the answer with `structured(...)`, which fills both `structuredContent` and its text
  rendering.
- Take a read lock for reads and a write lock for writes, and **keep the lock window
  minimal** — release it before doing anything else.
- Call `app_core::*` for all business logic.

```rust
use rmcp::ErrorData;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use uuid::Uuid;

use super::models::{GetWidgetParams, GetWidgetResult};
use crate::error::{McpError, into_tool_result, structured};
use crate::server::McpServer;

/// Widget lookups.
#[tool_router(router = widget_tool_router, vis = "pub(crate)")]
impl McpServer {
    /// Read one widget.
    #[tool(description = "Read one widget by its UUID. Returns its name, and fails when \
no widget has that id.")]
    async fn get_widget(
        &self,
        Parameters(GetWidgetParams { widget_id }): Parameters<GetWidgetParams>,
    ) -> Result<CallToolResult, ErrorData> {
        into_tool_result(self.read_widget(widget_id).await)
    }

    async fn read_widget(&self, widget_id: Uuid) -> Result<CallToolResult, McpError> {
        let widget = {
            let database = self.state.database.read().await;
            app_core::widget::get_widget(&*database, widget_id).await?
        };
        structured(GetWidgetResult::from(widget))
    }
}
```

### Errors

Two failure modes, and the choice matters because only one reaches the model:

- **Tool-level** — return a `McpError` and let `into_tool_result` render it. The tool ran
  and did not work (nothing matched, upstream refused). The client shows your message.
- **Protocol-level** — `Err(ErrorData)`. The request could not be processed at all. Clients
  render these opaquely, so the caller never reads your message.

Almost every case is the first. Add a `McpError` variant rather than reaching for
`ErrorData`, and map its caller-visible message in
[error/response.rs](../../../crates/mcp/src/error/response.rs) — the detailed trace stays in
the logs, exactly as `api`'s `ApiError` does.

## 4. Register the module

Add it to [tools/mod.rs](../../../crates/mcp/src/tools/mod.rs):

```rust
pub(crate) mod widget; // add this line
```

## 5. Register the router

In [server.rs](../../../crates/mcp/src/server.rs), sum the group's router into the
`#[tool_handler]` attribute:

```rust
#[tool_handler(router = (Self::ping_tool_router()
    + Self::user_tool_router()
    + Self::widget_tool_router()))]
impl ServerHandler for McpServer {}
```

The outer parentheses are load-bearing: the macro pastes this expression in front of a
method call, and a bare `a() + b()` would bind that call to the right operand only.

**A tool that is written but not summed here does not exist.** This is the MCP counterpart
of `api`'s `routes/router.rs`, and the one place to look to see everything exposed.

## 6. Widen the state only if you must

A tool reaches services through `self.state`, an
[`McpState`](../../../crates/mcp/src/state.rs). If yours needs a service that is not there,
add the field — and remember that doing so widens what *every* future tool can reach. Add it
together with the tool that needs it, never in advance.

`api` builds `McpState` in
[routes/router.rs](../../../crates/api/src/routes/router.rs); a new field must be passed
there too.

## 7. Flag a breaking change

Parameter and result types are a public contract. If you **removed**, **renamed** or
**changed the type of** a field — or renamed a tool — the commit needs a `BREAKING CHANGES:`
footer listing each one, prefixed `MCP:`. See Skill(commit-messages).

## Checklist

```bash
./scripts/test_units.sh   # the crate's tests, including the endpoint handshake
./scripts/test_lint.sh
```

```bash
# The tool is listed by a running backend
source .env && cargo run -p backend &
curl -sS http://localhost:8080/mcp \
  -H 'Authorization: <api key>' \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

- [ ] The tool calls `app_core` and holds no domain logic.
- [ ] Its description and every field's doc comment are written for the model that will
      read them.
- [ ] The result carries nothing the caller did not ask for.
- [ ] The group's router is summed in `server.rs`.
