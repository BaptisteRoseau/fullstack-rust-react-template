import { I18nProvider } from '@lingui/react'
import type { Preview } from '@storybook/react-vite'
import React, { useEffect, useState } from 'react'
import { MemoryRouter } from 'react-router'

import { defaultLocale, i18n, loadLocale } from '../src/i18n'
import '../src/css/main.scss'

function WithI18n({ children }: { children: React.ReactNode }) {
    const [isReady, setIsReady] = useState(false)

    useEffect(() => {
        void loadLocale(defaultLocale).then(() => setIsReady(true))
    }, [])

    if (!isReady) {
        return null
    }

    return <I18nProvider i18n={i18n}>{children}</I18nProvider>
}

const preview: Preview = {
    globalTypes: {
        theme: {
            description: 'Colour theme',
            defaultValue: 'light',
            toolbar: {
                icon: 'circlehollow',
                items: ['light', 'dark'],
            },
        },
    },
    decorators: [
        (Story, context) => {
            document.documentElement.dataset.theme = context.globals.theme
            return (
                <WithI18n>
                    <MemoryRouter>
                        <Story />
                    </MemoryRouter>
                </WithI18n>
            )
        },
    ],
}

export default preview
