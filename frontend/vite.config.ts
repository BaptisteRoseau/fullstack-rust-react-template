/// <reference types="vite/client" />

import { transformAsync } from '@babel/core'
import { lingui } from '@lingui/vite-plugin'
import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import type { Plugin } from 'vite'
import { defineConfig } from 'vitest/config'
// import { defineConfig } from 'vite'

// @vitejs/plugin-react v6 transforms JSX with oxc and ignores the `babel` option,
// so the Lingui macros must be expanded by a dedicated pre-transform pass.
function linguiMacro(): Plugin {
    return {
        name: 'lingui-macro',
        enforce: 'pre',
        async transform(code, id) {
            const [filepath] = id.split('?')
            if (
                filepath.includes('/node_modules/') ||
                !/\.[jt]sx?$/.test(filepath)
            ) {
                return null
            }
            if (!code.includes('@lingui/')) {
                return null
            }
            const parserPlugins = filepath.endsWith('.ts')
                ? ['typescript']
                : ['jsx', 'typescript']
            const result = await transformAsync(code, {
                filename: filepath,
                babelrc: false,
                configFile: false,
                sourceMaps: true,
                parserOpts: { plugins: parserPlugins },
                plugins: ['@lingui/babel-plugin-lingui-macro'],
            })
            if (!result?.code) {
                return null
            }
            return { code: result.code, map: result.map }
        },
    }
}

export default defineConfig({
    base: './',
    plugins: [linguiMacro(), react(), lingui(), tailwindcss()],
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
