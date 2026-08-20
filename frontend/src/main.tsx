import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import { App } from './App'
import './css/main.scss'
import { loadLocale } from './i18n'
import { storedLocale } from './stores/locale'
import { useTheme } from './stores/theme'
import { enableMocking } from './test-utils/enableMocking'

const root = document.getElementById('root')
if (!root) {
    throw new Error('No root element found')
}

useTheme.getState().setTheme(useTheme.getState().theme)

void Promise.all([enableMocking(), loadLocale(storedLocale())]).then(() => {
    createRoot(root).render(
        <StrictMode>
            <App />
        </StrictMode>,
    )
})
