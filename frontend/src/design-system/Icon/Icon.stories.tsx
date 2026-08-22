import type { Meta, StoryObj } from '@storybook/react-vite'

import {
    BoltIcon,
    CheckIcon,
    ChevronDownIcon,
    CloseIcon,
    CopyIcon,
    KeyIcon,
    LayersIcon,
    LogoutIcon,
    MonitorIcon,
    MoonIcon,
    PlusIcon,
    ShieldIcon,
    SunIcon,
    TrashIcon,
    UserIcon,
} from './Icon'

const meta = {
    title: 'Design System/Icon',
    component: CheckIcon,
} satisfies Meta<typeof CheckIcon>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { size: 24 } }

export const AllIcons: Story = {
    render: () => (
        <div style={{ display: 'flex', gap: 16, flexWrap: 'wrap' }}>
            <BoltIcon size={24} />
            <CheckIcon size={24} />
            <ChevronDownIcon size={24} />
            <CloseIcon size={24} />
            <CopyIcon size={24} />
            <KeyIcon size={24} />
            <LayersIcon size={24} />
            <LogoutIcon size={24} />
            <MonitorIcon size={24} />
            <MoonIcon size={24} />
            <PlusIcon size={24} />
            <ShieldIcon size={24} />
            <SunIcon size={24} />
            <TrashIcon size={24} />
            <UserIcon size={24} />
        </div>
    ),
}
