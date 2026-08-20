import type { Meta, StoryObj } from '@storybook/react-vite'

import { TrashIcon } from '@/design-system/Icon'

import { Button } from './Button'
import { IconButton } from './IconButton'

const meta = {
    title: 'Design System/Button',
    component: Button,
} satisfies Meta<typeof Button>

export default meta

type Story = StoryObj<typeof meta>

export const Primary: Story = { args: { children: 'Save' } }
export const Secondary: Story = {
    args: { children: 'Cancel', variant: 'secondary' },
}
export const Danger: Story = { args: { children: 'Delete', variant: 'danger' } }
export const Disabled: Story = { args: { children: 'Save', disabled: true } }

export const AllVariants: Story = {
    args: { children: 'Button' },
    render: () => (
        <div style={{ display: 'flex', gap: 12, alignItems: 'center' }}>
            <Button>Primary</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="ghost">Ghost</Button>
            <Button variant="danger">Danger</Button>
            <IconButton aria-label="Delete" variant="ghost">
                <TrashIcon />
            </IconButton>
        </div>
    ),
}
