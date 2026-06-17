import type { Preview } from '@storybook/react-vite'
import React from 'react'
import { MemoryRouter } from 'react-router'
import '../src/index.css'

const preview: Preview = {
    decorators: [
        (Story) => (
            <MemoryRouter>
                <Story />
            </MemoryRouter>
        ),
    ],
}

export default preview
