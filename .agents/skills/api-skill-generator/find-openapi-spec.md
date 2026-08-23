# Find OpenAPI Specification

To find an OpenAPI specification, look for it in these places (in YAML or JSON format):

1. A running instance of the service (if you have a reachable base URL), at a
   well-known spec path — see [Well-known paths](#well-known-paths) below.
2. The official developer documentation of the service.
3. The GitHub repository of the service (or of its documentation).
4. A plain web search.

Look for `openapi.json`, `openapi.yml`, or `openapi.yaml` files (or the paths listed
below).

Once the URL of the specification is found, do not save it locally and do not read its
full contents. Fetch only enough to validate it (see
[Validating a candidate](#validating-a-candidate)), then pass the URL directly as input
to `openapi2skill`.

## Well-known paths

If the service is running and you have its base URL (e.g. a local instance or a Docker
container), try these conventional paths before searching the web. They are often served
directly by the framework:

- `/openapi.json`, `/openapi.yaml`
- `/v3/api-docs` (Spring / springdoc)
- `/swagger/v1/swagger.json` (ASP.NET / Swashbuckle)
- `/api-docs`, `/swagger.json`
- `/docs`, `/swagger`, `/redoc` — these are usually HTML UIs (Swagger UI / ReDoc); open
  them only to discover the spec URL they reference, not to read the whole page.

## OpenAPI version

`openapi2skill` only supports OpenAPI **3.x**. You are looking for a **v3** spec: when you
validate a candidate, confirm it declares an `openapi` field matching `^3\.`.

If you find a **Swagger 2.0** spec instead (it declares `swagger: "2.0"` and no `openapi`
field), **do not stop there** — keep searching for a native v3 spec, which is always
preferable. Only fall back to converting the v2 spec if the search for a v3 one is
unsuccessful.

When you do need to convert, use a converter **only if it is already available** — for
example [`swagger2openapi`](https://github.com/Mermade/oas-kit):

```bash
# Example only — run this only if the tool is already installed.
swagger2openapi <swagger-2.0-url-or-file> -o openapi.json
```

Do **not** install a converter without the user's agreement: if none is available, ask the
user before installing one. If conversion is not possible, treat the search as failed and
report it (see [When to stop](#when-to-stop)).

Then use the converted `openapi.json` as input to `openapi2skill`.

## Delegating the web search

The web search (documentation / GitHub / plain search) is a good candidate to delegate to
a **cheaper, more cost-efficient subagent** if your tooling supports it. Instruct that
subagent to return **only the spec URL** (a single line), nothing else — no page content,
no summary. This keeps the expensive context focused on generating the skill.

## Best practices

The goal is to find a **link to the spec file** while spending as few tokens as possible.
The following commands are **advised but not mandatory** — use them when they help avoid
reading large HTML or spec bodies into context; skip them when a URL is already obvious.

Discover candidate links from a docs page without reading the whole page:

```bash
# List links whose href or text mentions OpenAPI / Swagger (case-insensitive)
curl -sL <docs-url> | grep -ioE '(href|src)="[^"]*"' | grep -iE 'openapi|swagger|api-docs'
```

```bash
# Or list all matching absolute/relative URLs found anywhere in the page
curl -sL <docs-url> | grep -ioE 'https?://[^"'\'' ]*(openapi|swagger|api-docs)[^"'\'' ]*'
```

### Validating a candidate

Once a file URL seems found, do not read the whole file. Instead, check its head only and
confirm it looks like a 3.x OpenAPI document:

```bash
# JSON: show the openapi version field without downloading the whole file
curl -sL <spec-url> | head -c 2000 | grep -oE '"openapi"[[:space:]]*:[[:space:]]*"3\.[^"]*"'
```

```bash
# YAML: same idea for a YAML spec
curl -sL <spec-url> | head -n 20 | grep -iE '^openapi:[[:space:]]*"?3\.'
```

A non-empty match confirms a valid OpenAPI 3.x spec; pass its URL to `openapi2skill`.

## Access and failure handling

If running in a sandboxed environment and network calls to the documentation are denied,
ask the user to temporarily allow access to it.

### When to stop

Do not search endlessly. If the specification cannot be found anywhere, stop and tell the
user that skill generation failed. Include the list of places you searched (URLs and
well-known paths tried) so they can point you to the right source.
