import type { StorybookConfig } from '@storybook/react-vite'
import type { InlineConfig } from 'vite'

const config: StorybookConfig = {
    stories: ['../src/**/*.stories.@(js|jsx|ts|tsx)'],
    addons: ['@storybook/addon-links', '@storybook/addon-a11y'],
    framework: {
        name: '@storybook/react-vite',
        options: {},
    },
    docs: {
        autodocs: 'tag',
    },
    typescript: {
        reactDocgen: 'react-docgen-typescript',
        reactDocgenTypescriptOptions: {
            include: ['../src/**/*.{ts,tsx}'],
        },
    },
    viteFinal: (config: InlineConfig) => ({
        ...config,
        build: { ...config.build, chunkSizeWarningLimit: 1000 },
    }),
}

export default config
