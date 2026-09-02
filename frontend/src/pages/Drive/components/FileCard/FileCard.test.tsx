import { screen } from '@testing-library/react'

import { useApiThumbnail } from '@/api/hooks/useApiThumbnail'
import { buildDriveFile } from '@/test-utils/fixtures/drive'
import { render } from '@/test-utils/render'

import { FileCard } from './FileCard'

vi.mock('@/api/hooks/useApiThumbnail')

beforeEach(() => {
    vi.mocked(useApiThumbnail).mockReturnValue({
        url: null,
        error: undefined,
        isLoading: false,
    })
})

it('renders the thumbnail as an image once it is there', () => {
    vi.mocked(useApiThumbnail).mockReturnValue({
        url: 'blob:thumbnail',
        error: undefined,
        isLoading: false,
    })

    render(
        <FileCard
            file={buildDriveFile({
                name: 'photo.png',
                mimeType: 'image/png',
                hasThumbnail: true,
            })}
            destinations={[]}
        />,
    )

    expect(
        screen.getByRole('button', { name: 'Preview photo.png' }),
        `expected the card to open a preview, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByRole('presentation'),
        'a file with a thumbnail must show the image itself, not an icon',
    ).toHaveAttribute('src', 'blob:thumbnail')
})

it('shows what the compression saved', () => {
    render(
        <FileCard
            file={buildDriveFile({
                name: 'notes.txt',
                sizeBytes: 1000,
                storedSizeBytes: 250,
            })}
            destinations={[]}
        />,
    )

    expect(
        screen.getByText('−75%'),
        `expected the saving to be shown, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('falls back to a type icon without a thumbnail', () => {
    render(
        <FileCard
            file={buildDriveFile({ name: 'notes.txt', hasThumbnail: false })}
            destinations={[]}
        />,
    )

    expect(
        screen.queryByRole('presentation'),
        'a file with no thumbnail must not render an empty image',
    ).not.toBeInTheDocument()
})
