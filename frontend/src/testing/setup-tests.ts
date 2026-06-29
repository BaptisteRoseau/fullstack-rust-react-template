import '@testing-library/jest-dom/vitest'

import { defaultLocale, loadLocale } from '@/i18n'
import { initializeDb, resetDb } from '@/testing/mocks/db'
import { server } from '@/testing/mocks/server'

vi.mock('zustand')

await loadLocale(defaultLocale)

beforeAll(() => server.listen({ onUnhandledRequest: 'error' }))
afterAll(() => server.close())
beforeEach(() => {
    class ResizeObserverMock {
        observe = vi.fn()
        unobserve = vi.fn()
        disconnect = vi.fn()
    }

    vi.stubGlobal('ResizeObserver', ResizeObserverMock)

    window.btoa = (str: string) => Buffer.from(str, 'binary').toString('base64')
    window.atob = (str: string) => Buffer.from(str, 'base64').toString('binary')

    initializeDb()
})
afterEach(() => {
    server.resetHandlers()
    resetDb()
})
