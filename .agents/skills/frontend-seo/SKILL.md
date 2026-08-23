---
name: frontend-seo
description: Use when publishing a route, changing the site origin, name or security contact, or fixing a seo:check failure.
---

# SEO and agent files

Five files are **generated**, never written by hand: `robots.txt`, `sitemap.xml`,
`site.webmanifest`, `llms.txt`, `.well-known/security.txt`. One config feeds one pure renderer,
called from an npm script that writes into `public/` and a Vite plugin that overwrites `dist/` with
the deployment origin at build time. See
[frontend/public/README.md](../../../frontend/public/README.md) for what each file is.

**Never edit a file under `public/` that this skill lists as generated.** Change `seo.config.ts` or
the templates in `scripts/generate-seo-files.ts`, then regenerate. `seo:check` compares the
committed files against a fresh render byte-for-byte and fails on any hand edit.

## 1. Add the path to the router

Add it to `PATHS` in `src/router/constants.ts` — Skill(frontend-page).

## 2. Opt it into the sitemap, or disallow it

`PUBLIC_PATHS` in [seo.config.ts](../../../frontend/seo.config.ts) opts **in**: a new `PATHS` entry
stays unpublished until it is listed there.

- If the route renders without a session, add it to `PUBLIC_PATHS`, then add its name and note to
  `PAGE_LABELS` in `scripts/generate-seo-files.ts` — the record is typed against `PUBLIC_PATHS`, so
  skipping this is a compile error.
- If the route requires a session, leave it out of `PUBLIC_PATHS` and add a `Disallow:` line to the
  `robots.txt` template instead.

## 3. Regenerate and verify

```bash
bun run seo:generate
bun run seo:check
bun run check-types
```

## 4. Commit

Commit the regenerated `public/` files in the same commit as the route.

Changing the generator itself — its purity rules, or what is deliberately out of scope — is rare;
read [internals.md](./internals.md) only when you need to do that.

## Config and deployment

Every field in `seo.config.ts` has a default and an env override: `SEO_SITE_URL` (falls back to
`VITE_APP_APP_URL`), `SEO_SITE_NAME`, `SEO_SITE_DESCRIPTION`, `SEO_THEME_COLOR`,
`SEO_SECURITY_CONTACT`, `SEO_SECURITY_POLICY_URL` (omits the `Policy:` line when unset),
`SEO_SECURITY_EXPIRES`. Read only at module level, under the `SEO_*` prefix — never `VITE_APP_*`,
which is a build-time Node var that must not reach the client bundle.

The committed copies carry the `http://localhost:3000` placeholder; the real origin is injected at
build time. Every deployment must:

- Pass `SEO_SITE_URL` to `bun run build`.
- Replace the placeholder `security@example.com` via `SEO_SECURITY_CONTACT`, and bump
  `securityExpires` before it lapses — `seo:check` warns under 90 days and fails once expired.

## Checklist

```bash
bun run seo:generate && bun run seo:check
```

- [ ] `PAGE_LABELS` has an entry for every path in `PUBLIC_PATHS`, or the build fails to compile.
- [ ] An authenticated route has a `Disallow:` line instead of a sitemap entry.
