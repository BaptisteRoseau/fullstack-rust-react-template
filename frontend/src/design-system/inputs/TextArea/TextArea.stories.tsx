import type { Meta, StoryObj } from '@storybook/react-vite'

import { TextArea } from './TextArea'

const meta = {
    title: 'Design System/inputs/TextArea',
    component: TextArea,
} satisfies Meta<typeof TextArea>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { placeholder: 'Tell us about you' } }
