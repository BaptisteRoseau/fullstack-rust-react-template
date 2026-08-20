import '@testing-library/jest-dom/vitest'

import { defaultLocale, loadLocale } from '@/i18n'
import { initializeDb, resetDb } from '@/test-utils/mocks/db'
import { server } from '@/test-utils/server'

await loadLocale(defaultLocale)

beforeAll(() => server.listen({ onUnhandledRequest: 'error' }))
afterAll(() => server.close())

beforeEach(async () => {
    class ResizeObserverMock {
        observe = vi.fn()
        unobserve = vi.fn()
        disconnect = vi.fn()
    }
    vi.stubGlobal('ResizeObserver', ResizeObserverMock)
    await initializeDb()
})

afterEach(() => {
    server.resetHandlers()
    resetDb()
    vi.clearAllMocks()
})
