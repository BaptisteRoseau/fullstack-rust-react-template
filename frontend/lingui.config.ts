import { defineConfig } from '@lingui/conf'
import { formatter } from '@lingui/format-po'

export default defineConfig({
    locales: ['en', 'fr'],
    sourceLocale: 'en',
    catalogs: [
        {
            path: 'src/i18n/locales/{locale}/messages',
            include: ['src/**'],
        },
    ],
    format: formatter({ lineNumbers: false }),
    compileNamespace: 'ts',
})
