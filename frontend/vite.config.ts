/// <reference types="vite/client" />

import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import { lingui } from '@lingui/vite-plugin'
import { defineConfig } from 'vitest/config'

export default defineConfig({
    base: './',
    plugins: [react(), tailwindcss(), lingui()],
    resolve: {
        tsconfigPaths: true,
    },
    server: {
        port: 3000,
    },
    preview: {
        port: 3000,
    },
    test: {
        globals: true,
        environment: 'jsdom',
        setupFiles: './src/testing/setup-tests.ts',
        exclude: ['**/node_modules/**', '**/e2e/**'],
        coverage: {
            include: ['src/**'],
        },
    },
    optimizeDeps: { exclude: ['fsevents'] },
    build: {
        chunkSizeWarningLimit: 1000,
        rollupOptions: {
            external: ['fs/promises'],
        },
    },
})
