import type { Meta, StoryObj } from '@storybook/react-vite'

import { SelectInput } from './SelectInput'

const meta = {
    title: 'Design System/inputs/SelectInput',
    component: SelectInput,
} satisfies Meta<typeof SelectInput>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
    args: {
        options: [
            { value: 'en', label: 'English' },
            { value: 'fr', label: 'Français' },
        ],
    },
}
