# `e2e/`

End-to-end tests with **Playwright** (config in `playwright.config.ts`). They run against the
standalone MSW **mock server** (`mock-server.ts` at the frontend root), so no real backend is needed.

```
e2e/
├── tests/          # Playwright specs (critical user journeys)
└── screenshots.ts  # screenshot helper
```

## Run

```bash
bun run test:e2e   # starts the mock server (pm2) then runs `playwright test`
```

Reserve e2e for a few critical journeys (auth, create/delete a discussion). Most coverage belongs in
the faster Vitest integration tests (`src/**/__tests__`, see `.claude/skills/frontend-react-testing`).
The mock data/handlers are shared with the unit tests — see `src/testing/mocks/`.
