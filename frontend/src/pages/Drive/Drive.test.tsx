import { screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

import { useApiCreateDirectory } from '@/api/hooks/useApiCreateDirectory'
import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { useApiEntries } from '@/api/hooks/useApiEntries'
import { useApiThumbnail } from '@/api/hooks/useApiThumbnail'
import { useApiUploadFile } from '@/api/hooks/useApiUploadFile'
import {
    buildDriveDirectory,
    buildDriveEntries,
    buildDriveFile,
} from '@/test-utils/fixtures/drive'
import { render } from '@/test-utils/render'

import { Drive } from './Drive'

vi.mock('@/api/hooks/useApiEntries')
vi.mock('@/api/hooks/useApiCreateDirectory')
vi.mock('@/api/hooks/useApiUploadFile')
vi.mock('@/api/hooks/useApiThumbnail')
vi.mock('@/api/hooks/useApiCurrentUser')

type EntriesResult = ReturnType<typeof useApiEntries>

function mockEntries(result: Partial<EntriesResult>) {
    vi.mocked(useApiEntries).mockReturnValue({
        data: undefined,
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
        ...result,
    } as EntriesResult)
}

const createTrigger = vi.fn()
const uploadTrigger = vi.fn()

beforeEach(() => {
    createTrigger.mockResolvedValue(buildDriveDirectory())
    uploadTrigger.mockResolvedValue(buildDriveFile())
    vi.mocked(useApiCreateDirectory).mockReturnValue({
        trigger: createTrigger,
        isMutating: false,
    } as unknown as ReturnType<typeof useApiCreateDirectory>)
    vi.mocked(useApiUploadFile).mockReturnValue({
        trigger: uploadTrigger,
        isMutating: false,
    } as unknown as ReturnType<typeof useApiUploadFile>)
    vi.mocked(useApiThumbnail).mockReturnValue({
        url: null,
        error: undefined,
        isLoading: false,
    })
    vi.mocked(useApiCurrentUser).mockReturnValue({
        data: undefined,
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    } as unknown as ReturnType<typeof useApiCurrentUser>)
    mockEntries({ data: buildDriveEntries() })
})

it('lists the folders and the files it was given', () => {
    mockEntries({
        data: buildDriveEntries({
            directories: [buildDriveDirectory({ name: 'Invoices' })],
            files: [buildDriveFile({ name: 'notes.txt', sizeBytes: 2048 })],
        }),
    })

    render(<Drive />)

    expect(
        screen.getByText('Invoices'),
        `expected the folder card, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByText('notes.txt'),
        `expected the file card, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByText('2.0 KB'),
        `expected the formatted size, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('shows a spinner while the folder loads', () => {
    mockEntries({ data: undefined, isLoading: true })

    render(<Drive />)

    expect(
        screen.getByRole('status', { name: 'Loading' }),
        `expected the loading state, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('shows the empty state for a folder with nothing in it', () => {
    render(<Drive />)

    expect(
        screen.getByText('This folder is empty. Upload a file to start.'),
        `expected the empty state, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('reports a folder that could not be loaded', () => {
    mockEntries({ data: undefined, error: new Error('boom') })

    render(<Drive />)

    expect(
        screen.getByRole('alert'),
        `expected the error state, got: ${document.body.textContent}`,
    ).toHaveTextContent('This folder could not be loaded.')
})

it('creates a folder with the name the user typed', async () => {
    render(<Drive />)

    await userEvent.click(screen.getByRole('button', { name: 'New folder' }))
    const dialog = screen.getByRole('dialog')
    await userEvent.type(within(dialog).getByLabelText('Name'), 'Invoices')
    await userEvent.click(
        within(dialog).getByRole('button', { name: 'Create folder' }),
    )

    expect(
        createTrigger.mock.calls[0]?.[0],
        `expected the typed name at the current level, got ${JSON.stringify(createTrigger.mock.calls[0]?.[0])}`,
    ).toEqual({ name: 'Invoices', parentId: null })
})

it('uploads the file the user picked', async () => {
    render(<Drive />)

    const file = new File(['hello'], 'report.txt', { type: 'text/plain' })
    await userEvent.upload(
        screen.getByLabelText('Choose files to upload'),
        file,
    )

    expect(
        uploadTrigger.mock.calls[0]?.[0]?.file?.name,
        `expected report.txt to be uploaded, got ${JSON.stringify(uploadTrigger.mock.calls[0]?.[0]?.file?.name)}`,
    ).toBe('report.txt')
})

it('shows the signed-in user their own id, because sharing needs it', () => {
    vi.mocked(useApiCurrentUser).mockReturnValue({
        data: {
            id: 'user-42',
            email: 'ada@example.com',
            firstName: 'Ada',
            lastName: 'Lovelace',
            role: 'USER',
            teamId: '',
            createdAt: new Date(),
        },
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    } as unknown as ReturnType<typeof useApiCurrentUser>)

    render(<Drive />)

    expect(
        screen.getByText('user-42'),
        `expected the user id to be reachable, got: ${document.body.textContent}`,
    ).toBeVisible()
})
