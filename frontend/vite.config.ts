/// <reference types="vite/client" />

import path from 'node:path'

import { transformAsync } from '@babel/core'
import { lingui } from '@lingui/vite-plugin'
import react from '@vitejs/plugin-react'
import type { Plugin } from 'vite'
import svgr from 'vite-plugin-svgr'
import { defineConfig } from 'vitest/config'

import { writeSeoFiles } from './scripts/generate-seo-files'
import { seoConfig } from './seo.config'

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
            const result = await transformAsync(code, {
                filename: filepath,
                babelrc: false,
                configFile: false,
                sourceMaps: true,
                parserOpts: {
                    plugins: filepath.endsWith('.ts')
                        ? ['typescript']
                        : ['jsx', 'typescript'],
                },
                plugins: ['@lingui/babel-plugin-lingui-macro'],
            })
            if (!result?.code) {
                return null
            }
            return { code: result.code, map: result.map }
        },
    }
}

/**
 * Overwrites the committed placeholder SEO files in the build output with ones
 * carrying the configured origin (`SEO_SITE_URL`). Writing in `closeBundle`
 * rather than `generateBundle` matters: Vite copies `public/` into `outDir`
 * during the write phase, which would clobber emitted assets.
 */
function seoFiles(): Plugin {
    let outDir = ''

    return {
        name: 'seo-files',
        apply: 'build',
        configResolved(config) {
            outDir = path.resolve(config.root, config.build.outDir)
        },
        async closeBundle() {
            await writeSeoFiles(outDir, seoConfig)
        },
    }
}

export default defineConfig({
    base: './',
    plugins: [linguiMacro(), react(), lingui(), svgr(), seoFiles()],
    resolve: {
        alias: { '@': path.resolve(__dirname, 'src') },
    },
    css: {
        preprocessorOptions: {
            scss: {
                loadPaths: [path.resolve(__dirname, 'src/css')],
            },
        },
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
        setupFiles: './src/test-utils/setup-tests.ts',
        exclude: ['**/node_modules/**', '**/e2e/**'],
        css: { modules: { classNameStrategy: 'non-scoped' } },
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
