import { test as base } from '@playwright/test'

const MOCK_API_URL = 'http://localhost:8081'

export const test = base.extend({
    page: async ({ page, request }, use) => {
        await request.post(`${MOCK_API_URL}/api/__reset`)
        await use(page)
    },
})

export { expect, type Page } from '@playwright/test'
