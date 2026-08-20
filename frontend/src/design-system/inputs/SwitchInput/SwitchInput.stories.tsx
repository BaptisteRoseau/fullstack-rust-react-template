import type { Meta, StoryObj } from '@storybook/react-vite'

import { SwitchInput } from './SwitchInput'

const meta = {
    title: 'Design System/inputs/SwitchInput',
    component: SwitchInput,
} satisfies Meta<typeof SwitchInput>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { 'aria-label': 'Dark theme' } }
export const Checked: Story = {
    args: { 'aria-label': 'Dark theme', defaultChecked: true },
}
