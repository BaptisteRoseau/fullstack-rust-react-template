# `components/errors/`

Error UI for `react-error-boundary` boundaries.

- `main.tsx` — `MainErrorFallback`, the fallback wired into the root boundary in `app/provider.tsx`.

Render error boundaries at the **feature level** too (not just the app root) so a single broken
feature degrades gracefully. Use this fallback (or a feature-specific one following the same shape)
as the `FallbackComponent`.
