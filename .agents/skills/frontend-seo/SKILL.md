---
name: frontend-seo
description: How to regenerate and extend the generated SEO and agent files — robots.txt, sitemap.xml, llms.txt, site.webmanifest, security.txt. Use this when publishing a route, changing the site origin, name or security contact, or fixing a seo:check failure.
---

# SEO and agent files

Five files are **generated**, never written by hand. One config feeds one pure renderer, which is
called from two places: an npm script that writes the committed placeholders into `public/`, and a
Vite plugin that overwrites them in `dist/` with the deployment origin.

```
frontend/
├── seo.config.ts                      # Zod-validated config + PUBLIC_PATHS (opt-in route list)
├── scripts/
│   ├── generate-seo-files.ts          # renderSeoFiles() — every file template lives here
│   ├── generate-seo-files.cli.ts      # writes into public/ (bun run seo:generate)
│   └── generate-seo-files.test.ts     # 8 drift + conformance checks (bun run seo:check)
├── vite.config.ts                     # seoFiles() plugin — rewrites dist/ at build
├── index.html                         # carries only <link rel="manifest">
└── public/                            # GENERATED — do not hand-edit
    ├── robots.txt, sitemap.xml, site.webmanifest, llms.txt
    ├── .well-known/security.txt
    └── README.md                      # documents the above; keep in sync
```

**Never edit a file under `public/` that this skill lists as generated.** Change `seo.config.ts` or
the templates in `scripts/generate-seo-files.ts`, then regenerate. `seo:check` compares the
committed files against a fresh render byte-for-byte and fails on any hand edit.

## Commands

```bash
bun run seo:generate                                # render → frontend/public/
bun run seo:check                                   # drift + conformance spec
bun run check-types                                 # covers seo.config.ts and scripts/
SEO_SITE_URL=https://example.test bun run build     # verify the dist/ origin override
npx eslint scripts seo.config.ts                    # `bun run lint` only covers src and e2e
```

## Checklist — publishing a route

1. Add the path to `PATHS` in `src/router/constants.ts` (see the `frontend-page` skill).
2. Add it to `PUBLIC_PATHS` in `seo.config.ts` — **only** if it renders without a session.
3. Add its name and note to `PAGE_LABELS` in `scripts/generate-seo-files.ts`; the record is typed
   against `PUBLIC_PATHS`, so skipping this is a compile error.
4. Leave an authenticated route out of `PUBLIC_PATHS` and add a `Disallow:` line to the
   `robots.txt` template instead.
5. Run `bun run seo:generate`.
6. Run `bun run seo:check`, then `bun run check-types`.
7. Commit the regenerated `public/` files in the same commit as the route.

`PUBLIC_PATHS` opts **in**: a new entry in `PATHS` stays unpublished until it is listed there.

## Rules for the generator

- Keep `renderSeoFiles` pure — no `fs`, no `process.env`, no clock. The drift check depends on it.
- **Never read the clock anywhere in the generator.** `securityExpires` is a pinned timestamp in
  `seo.config.ts`, never a computed offset; one `Date.now()` breaks the drift test permanently.
- End every rendered file with exactly one trailing newline.
- Escape XML entities in `sitemap.xml` URLs.
- Read env only at module level in `seo.config.ts`, under the `SEO_*` prefix. Never `VITE_APP_*` —
  these are build-time Node vars and must not reach the client bundle.
- Import `PATHS` relatively (`./src/router/constants`), not via `@/`. Vite bundles `vite.config.ts`
  with esbuild without applying `resolve.alias`, so an aliased import breaks the build.
- The plugin writes in `closeBundle`, not `generateBundle`. Vite copies `public/` into `outDir`
  during the write phase and would clobber emitted assets.
- Add no runtime dependency for this. `sitemap` and `vite-plugin-sitemap` scan `dist/` for HTML or
  bind to file-based routing, and handle neither `security.txt` nor `llms.txt`.

## Config and deployment

Every field in `seo.config.ts` has a default and an env override: `SEO_SITE_URL` (falls back to
`VITE_APP_APP_URL`), `SEO_SITE_NAME`, `SEO_SITE_DESCRIPTION`, `SEO_THEME_COLOR`,
`SEO_SECURITY_CONTACT`, `SEO_SECURITY_POLICY_URL` (omits the `Policy:` line when unset),
`SEO_SECURITY_EXPIRES`.

The committed copies carry the `http://localhost:3000` placeholder; the real origin is injected at
build time. Two things every deployment must do:

- Pass `SEO_SITE_URL` to `bun run build`.
- Replace the placeholder `security@example.com` via `SEO_SECURITY_CONTACT`, and bump
  `securityExpires` before it lapses — `seo:check` warns under 90 days and fails once expired.

## Tests

`scripts/generate-seo-files.test.ts` covers: byte-for-byte drift, the sitemap namespace and its
`<loc>` set, absolute URLs, exactly one `Sitemap:` line, `/user` disallowed with the AI-crawler
stanza left commented, RFC 9116 required fields, llms.txt v2 structure, and manifest icons that
exist on disk. Add a check whenever you add a generated file. Every assertion message must
interpolate the offending value.

The spec anchors on `process.cwd()`, not `import.meta.url` — jsdom rewrites the latter to a
non-file URL. `dist/` emission is verified manually with the build command above, not in the spec.

## Out of scope

Declined deliberately — do not add without being asked: Open Graph, Twitter Card, `rel="canonical"`
and JSON-LD (no SSR, so `index.html` carries only the manifest link); `hreflang` alternates (Lingui
`en`/`fr` share one URL); `llms-full.txt`, `humans.txt`, `ads.txt`, `browserconfig.xml`,
`opensearch.xml`. `apple-touch-icon.png` needs a new 180×180 asset — raise it, do not generate one.

## Related skills

`frontend-page` (routes and `PATHS`), `frontend-architecture`, `frontend-testing`.
