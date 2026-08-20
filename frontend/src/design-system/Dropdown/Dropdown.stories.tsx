import type { Meta, StoryObj } from '@storybook/react-vite'

import { Button } from '@/design-system/Button'

import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownLabel,
    DropdownSeparator,
    DropdownTrigger,
} from './Dropdown'

const meta = {
    title: 'Design System/Dropdown',
    component: DropdownContent,
} satisfies Meta<typeof DropdownContent>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
    render: () => (
        <Dropdown>
            <DropdownTrigger asChild>
                <Button variant="secondary">Open menu</Button>
            </DropdownTrigger>
            <DropdownContent>
                <DropdownLabel>ada@example.com</DropdownLabel>
                <DropdownSeparator />
                <DropdownItem>Profile</DropdownItem>
                <DropdownItem>Log out</DropdownItem>
            </DropdownContent>
        </Dropdown>
    ),
}
