import type { Meta, StoryObj } from '@storybook/react-vite'

import { TextInput } from './TextInput'

const meta = {
    title: 'Design System/inputs/TextInput',
    component: TextInput,
} satisfies Meta<typeof TextInput>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { placeholder: 'Jane Doe' } }
export const Invalid: Story = {
    args: { placeholder: 'Jane Doe', 'aria-invalid': true },
}
export const Disabled: Story = { args: { value: 'Jane Doe', disabled: true } }
