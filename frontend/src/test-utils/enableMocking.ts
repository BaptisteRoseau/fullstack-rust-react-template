import { env } from '@/config/env'

export async function enableMocking() {
    if (!env.ENABLE_API_MOCKING) {
        return
    }
    const { worker } = await import('./mocks/browser')
    const { initializeDb } = await import('./mocks/db')
    await initializeDb()
    await worker.start({ onUnhandledRequest: 'bypass' })
}
