import type { Meta, StoryObj } from '@storybook/react-vite'

import { Link } from './Link'

const meta = {
    title: 'Design System/Link',
    component: Link,
} satisfies Meta<typeof Link>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
    args: { children: 'Read the documentation', href: '#' },
}

export const Muted: Story = {
    args: { children: 'Privacy policy', href: '#', variant: 'muted' },
}
