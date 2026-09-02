import type { Meta, StoryObj } from '@storybook/react-vite'

import {
    BoltIcon,
    CheckIcon,
    ChevronDownIcon,
    ChevronRightIcon,
    CloseIcon,
    CopyIcon,
    DownloadIcon,
    EyeIcon,
    FileIcon,
    FileTextIcon,
    FolderIcon,
    ImageIcon,
    KeyIcon,
    LayersIcon,
    LogoutIcon,
    MonitorIcon,
    MoonIcon,
    MoveIcon,
    PencilIcon,
    PlusIcon,
    ShareIcon,
    ShieldIcon,
    SunIcon,
    TrashIcon,
    UploadIcon,
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
            <ChevronRightIcon size={24} />
            <CloseIcon size={24} />
            <CopyIcon size={24} />
            <DownloadIcon size={24} />
            <EyeIcon size={24} />
            <FileIcon size={24} />
            <FileTextIcon size={24} />
            <FolderIcon size={24} />
            <ImageIcon size={24} />
            <KeyIcon size={24} />
            <LayersIcon size={24} />
            <LogoutIcon size={24} />
            <MonitorIcon size={24} />
            <MoonIcon size={24} />
            <MoveIcon size={24} />
            <PencilIcon size={24} />
            <PlusIcon size={24} />
            <ShareIcon size={24} />
            <ShieldIcon size={24} />
            <SunIcon size={24} />
            <TrashIcon size={24} />
            <UploadIcon size={24} />
            <UserIcon size={24} />
        </div>
    ),
}
