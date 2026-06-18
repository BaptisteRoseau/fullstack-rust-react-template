# `components/seo/`

- `head.tsx` — `Head`, a thin wrapper around `react-helmet-async` that sets the document `<title>`
  (and meta) per screen. Re-exported via `index.ts`.

`Head` is rendered by the layouts (e.g. `ContentLayout`), so most pages get their title by passing
`title` to the layout. Use `Head` directly only for screens that don't go through a layout.

`react-helmet-async`'s `HelmetProvider` is mounted once in `app/provider.tsx`.
