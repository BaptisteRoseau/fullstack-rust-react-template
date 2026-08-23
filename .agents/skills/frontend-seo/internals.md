# SEO generator internals

Read this only when editing `scripts/generate-seo-files.ts` or `vite.config.ts`'s `seoFiles()`
plugin, not when publishing a route.

## Rules for the generator

- Keep `renderSeoFiles` pure — no `fs`, no `process.env`, no clock. The drift check depends on it.
- **Never read the clock anywhere in the generator.** `securityExpires` is a pinned timestamp in
  `seo.config.ts`, never a computed offset; one `Date.now()` breaks the drift test permanently.
- End every rendered file with exactly one trailing newline.
- Escape XML entities in `sitemap.xml` URLs.
- Import `PATHS` relatively (`./src/router/constants`), not via `@/`. Vite bundles
  `vite.config.ts` with esbuild without applying `resolve.alias`, so an aliased import breaks the
  build.
- The plugin writes in `closeBundle`, not `generateBundle`. Vite copies `public/` into `outDir`
  during the write phase and would clobber emitted assets.
- Add no runtime dependency for this. `sitemap` and `vite-plugin-sitemap` scan `dist/` for HTML or
  bind to file-based routing, and handle neither `security.txt` nor `llms.txt`.

## Tests

`scripts/generate-seo-files.test.ts` covers: byte-for-byte drift, the sitemap namespace and its
`<loc>` set, absolute URLs, exactly one `Sitemap:` line, `/user` disallowed with the AI-crawler
stanza left commented, RFC 9116 required fields, llms.txt v2 structure, and manifest icons that
exist on disk. Add a check whenever you add a generated file. Every assertion message must
interpolate the offending value.

The spec anchors on `process.cwd()`, not `import.meta.url` — jsdom rewrites the latter to a
non-file URL. `dist/` emission is verified manually with
`SEO_SITE_URL=https://example.test bun run build`, not in the spec.

## Out of scope

Declined deliberately — do not add without being asked: Open Graph, Twitter Card,
`rel="canonical"` and JSON-LD (no SSR, so `index.html` carries only the manifest link); `hreflang`
alternates (Lingui `en`/`fr` share one URL); `llms-full.txt`, `humans.txt`, `ads.txt`,
`browserconfig.xml`, `opensearch.xml`. `apple-touch-icon.png` needs a new 180×180 asset — raise it,
do not generate one.
