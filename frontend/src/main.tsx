import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './app/App.tsx'

const root = document.getElementById('root')
if (root) {
    createRoot(root).render(
        <StrictMode>
            <App />
        </StrictMode>
    )
} else {
    console.warn(
        'createRoot called in a non-browser environment. Skipping render.'
    )
}
