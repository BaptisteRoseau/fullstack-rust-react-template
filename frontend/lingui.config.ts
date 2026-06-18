import { defineConfig } from '@lingui/conf'

export default defineConfig({
    locales: ['en', 'fr'],
    sourceLocale: 'en',
    catalogs: [
        {
            path: 'src/i18n/locales/{locale}/messages',
            include: ['src/**'],
        },
    ],
    compileNamespace: 'ts',
})
