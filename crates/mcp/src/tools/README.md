# Tools

The MCP tools. Each group is one directory holding the tools and the types they take and
return — the same shape as [api's endpoints](../../../api/src/endpoints/README.md).

A tool takes its arguments, calls [app_core](../../../app_core), and serialises the result.
Business logic never lives here.

## Organization

```txt
<name>/
├── models.rs   # parameter and result types
├── tools.rs    # the tool functions, in an `impl McpServer` block
└── mod.rs      # only `pub(crate) mod tools;` and `pub(crate) mod models;`
```

A group with no arguments and no result type of its own may skip `models.rs`; see
[ping](./ping).

## Models

Named `<Tool><"Params" | "Result">`. For `get_user` they are `GetUserParams` and
`GetUserResult`.

Every model derives `Debug`, `JsonSchema` and `#[serde(rename_all = "camelCase")]`, plus:

| Kind | Additional derives |
| --- | --- |
| Parameters | `Deserialize` |
| Result | `Serialize` |

`JsonSchema` is what gives the tool its `inputSchema`, so **every field's doc comment is
read by the model** that decides how to call the tool. Write them for that reader.

Conversions from `app_core` and `database` models are implemented here as `From<T>`, so
tools stay free of mapping code.

## Tools

Each group is an inherent `impl McpServer` block carrying
`#[tool_router(router = <name>_tool_router, vis = "pub(crate)")]`. Rust allows an inherent
impl to be split across modules, which is what lets each group own its file while all of
them attach to the same server.

Tools return `Result<CallToolResult, ErrorData>` and build their answer through
[`structured`](../error/response.rs), which fills both the machine-readable
`structuredContent` and its text rendering.

The `#[tool(description = ...)]` text and the doc comment above a tool are what an
assistant reads before choosing it. Say what the tool returns and when it fails.

## Contract

These types are a public contract, like the API's. **Removing**, **renaming** or **changing
the type of** a parameter or result field breaks every client prompt built around it, and
the commit must carry a `BREAKING CHANGES:` footer listing each one, prefixed with `MCP:`.

## Exposure

Everything reachable here is reachable by an assistant, and through it by whoever is talking
to that assistant. Two consequences:

- A tool returns what the caller asked for and nothing more. `GetUserResult` drops
  `permissions` for that reason: it grants rather than describes.
- Adding a service to [`McpState`](../state.rs) widens what any future tool can reach, so
  add a field only together with the tool that needs it.

## Skills

- [backend-add-mcp-tool](../../../../.claude/skills/backend-add-mcp-tool/SKILL.md)
- [commit-messages](../../../../.claude/skills/commit-messages/SKILL.md)
