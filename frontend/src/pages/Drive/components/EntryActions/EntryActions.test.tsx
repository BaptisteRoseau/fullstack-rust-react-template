import { screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

import { useApiDeleteFile } from '@/api/hooks/useApiDeleteFile'
import { useApiDownloadFile } from '@/api/hooks/useApiDownloadFile'
import { buildDriveFile } from '@/test-utils/fixtures/drive'
import { render } from '@/test-utils/render'

import { EntryActions } from './EntryActions'

vi.mock('@/api/hooks/useApiDeleteFile')
vi.mock('@/api/hooks/useApiDeleteDirectory')
vi.mock('@/api/hooks/useApiDownloadFile')

const deleteTrigger = vi.fn()
const download = vi.fn()

beforeEach(() => {
    deleteTrigger.mockResolvedValue(undefined)
    download.mockResolvedValue(undefined)
    vi.mocked(useApiDeleteFile).mockReturnValue({
        trigger: deleteTrigger,
        isMutating: false,
    } as unknown as ReturnType<typeof useApiDeleteFile>)
    vi.mocked(useApiDownloadFile).mockReturnValue({
        download,
        isDownloading: false,
    })
})

const file = buildDriveFile({ id: 'file-42', name: 'notes.txt' })

function renderActions() {
    return render(
        <EntryActions
            kind="file"
            entryId={file.id}
            name={file.name}
            file={file}
            destinations={[]}
        />,
    )
}

it('asks for a confirmation before it deletes anything', async () => {
    renderActions()

    await userEvent.click(
        screen.getByRole('button', { name: 'Delete notes.txt' }),
    )

    expect(
        deleteTrigger,
        'opening the confirmation must not delete on its own',
    ).not.toHaveBeenCalled()
    expect(
        screen.getByRole('dialog'),
        `expected the confirmation, got: ${document.body.textContent}`,
    ).toHaveTextContent('This cannot be undone.')
})

it('deletes once the confirmation is accepted', async () => {
    renderActions()

    await userEvent.click(
        screen.getByRole('button', { name: 'Delete notes.txt' }),
    )
    await userEvent.click(
        within(screen.getByRole('dialog')).getByRole('button', {
            name: 'Delete',
        }),
    )

    expect(
        deleteTrigger.mock.calls.length,
        `expected 1 delete, got ${deleteTrigger.mock.calls.length}`,
    ).toBe(1)
})

it('downloads the file under its own name', async () => {
    renderActions()

    await userEvent.click(
        screen.getByRole('button', { name: 'Actions for notes.txt' }),
    )
    await userEvent.click(screen.getByRole('menuitem', { name: 'Download' }))

    expect(
        download.mock.calls[0],
        `expected the file id and name, got ${JSON.stringify(download.mock.calls[0])}`,
    ).toEqual(['file-42', 'notes.txt'])
})
