import { env } from '@/config/env'

/**
 * Starts the MSW browser worker. The `import.meta.env.DEV` guard is not
 * redundant with the env flag: it folds to `false` at build time, so Rollup
 * drops the dynamic imports below and keeps MSW, `@msw/data` and the handlers
 * out of the production bundle. The worker is a dev affordance anyway — e2e
 * goes through `mock-server.ts` instead.
 */
export async function enableMocking() {
    if (!import.meta.env.DEV || !env.ENABLE_API_MOCKING) {
        return
    }
    const { worker } = await import('./mocks/browser')
    const { initializeDb } = await import('./mocks/db')
    await initializeDb()
    await worker.start({ onUnhandledRequest: 'bypass' })
}
