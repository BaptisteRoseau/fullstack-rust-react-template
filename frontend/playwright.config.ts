import { defineConfig, devices } from '@playwright/test'

const PORT = 3000
const MOCK_API_PORT = 8081

const mockEnv = {
    VITE_APP_API_URL: `http://localhost:${MOCK_API_PORT}`,
    VITE_APP_ENABLE_API_MOCKING: 'false',
    VITE_APP_MOCK_API_PORT: String(MOCK_API_PORT),
    VITE_APP_URL: `http://localhost:${PORT}`,
}

export default defineConfig({
    testDir: './e2e',
    fullyParallel: false,
    forbidOnly: !!process.env.CI,
    retries: process.env.CI ? 2 : 0,
    workers: 1,
    reporter: process.env.CI ? 'html' : 'list',
    use: {
        baseURL: `http://localhost:${PORT}`,
        trace: 'on-first-retry',
    },
    projects: [
        {
            name: 'chromium',
            testMatch: /.*\.spec\.ts/,
            use: { ...devices['Desktop Chrome'] },
        },
    ],
    webServer: [
        {
            command: `bun run dev -- --port ${PORT}`,
            env: mockEnv,
            timeout: 30 * 1000,
            port: PORT,
            reuseExistingServer: !process.env.CI,
        },
        {
            command: 'bun run run-mock-server',
            env: mockEnv,
            timeout: 30 * 1000,
            port: MOCK_API_PORT,
            reuseExistingServer: !process.env.CI,
        },
    ],
})
