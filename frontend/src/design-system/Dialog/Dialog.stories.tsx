import type { Meta, StoryObj } from '@storybook/react-vite'

import { Button } from '@/design-system/Button'

import {
    DialogClose,
    DialogContent,
    DialogFooter,
    DialogRoot,
    DialogTrigger,
} from './Dialog'

const meta = {
    title: 'Design System/Dialog',
    component: DialogContent,
} satisfies Meta<typeof DialogContent>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
    args: { title: 'Revoke API key' },
    render: () => (
        <DialogRoot>
            <DialogTrigger asChild>
                <Button>Revoke</Button>
            </DialogTrigger>
            <DialogContent
                title="Revoke API key"
                description="This action cannot be undone."
            >
                <DialogFooter>
                    <DialogClose asChild>
                        <Button variant="secondary">Cancel</Button>
                    </DialogClose>
                    <Button variant="danger">Revoke</Button>
                </DialogFooter>
            </DialogContent>
        </DialogRoot>
    ),
}
