import type { Meta, StoryObj } from '@storybook/react-vite'

import { CheckboxInput } from './CheckboxInput'

const meta = {
    title: 'Design System/inputs/CheckboxInput',
    component: CheckboxInput,
} satisfies Meta<typeof CheckboxInput>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { label: 'read' } }
export const Checked: Story = { args: { label: 'write', defaultChecked: true } }
