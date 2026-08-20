import type { Meta, StoryObj } from '@storybook/react-vite'

import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from './Table'

const meta = {
    title: 'Design System/Table',
    component: Table,
} satisfies Meta<typeof Table>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = {
    render: () => (
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>Name</TableHead>
                    <TableHead>Created</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell>CI deploy key</TableCell>
                    <TableCell>March 3, 2026</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>Local dev</TableCell>
                    <TableCell>April 12, 2026</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    ),
}
