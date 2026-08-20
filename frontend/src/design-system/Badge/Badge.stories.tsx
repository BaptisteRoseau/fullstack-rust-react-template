import type { Meta, StoryObj } from '@storybook/react-vite'

import { Badge } from './Badge'

const meta = {
    title: 'Design System/Badge',
    component: Badge,
} satisfies Meta<typeof Badge>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { children: 'read' } }

export const AllVariants: Story = {
    args: { children: 'read' },
    render: () => (
        <div style={{ display: 'flex', gap: 8 }}>
            <Badge>neutral</Badge>
            <Badge variant="success">active</Badge>
            <Badge variant="warning">expiring</Badge>
            <Badge variant="danger">revoked</Badge>
        </div>
    ),
}
