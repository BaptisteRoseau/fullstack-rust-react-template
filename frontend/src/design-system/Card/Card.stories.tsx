import type { Meta, StoryObj } from '@storybook/react-vite'

import { Card } from './Card'

const meta = {
    title: 'Design System/Card',
    component: Card,
} satisfies Meta<typeof Card>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
    args: { children: 'Everything you need to ship a fullstack app.' },
}
