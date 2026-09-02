import { renderHook } from '@testing-library/react'
import { MemoryRouter } from 'react-router'

import { buildDriveDirectory } from '@/test-utils/fixtures/drive'

import { useDriveTrail } from './useDriveTrail'

function renderTrail(
    directoryId: string | undefined,
    directory: ReturnType<typeof buildDriveDirectory> | null,
    state?: unknown,
) {
    return renderHook(() => useDriveTrail(directoryId, directory), {
        wrapper: ({ children }) => (
            <MemoryRouter initialEntries={[{ pathname: '/drive', state }]}>
                {children}
            </MemoryRouter>
        ),
    })
}

it('is empty and complete at the root', () => {
    const { result } = renderTrail(undefined, null)

    expect(
        result.current,
        `expected an empty complete trail, got ${JSON.stringify(result.current)}`,
    ).toEqual({ entries: [], isComplete: true })
})

it('uses the trail the navigation carried', () => {
    const trail = [
        { id: 'dir-1', name: 'Invoices' },
        { id: 'dir-2', name: '2026' },
    ]

    const { result } = renderTrail(
        'dir-2',
        buildDriveDirectory({ id: 'dir-2', name: '2026', parentId: 'dir-1' }),
        { trail },
    )

    expect(
        result.current.entries.map((entry) => entry.name),
        `expected the walked trail, got ${JSON.stringify(result.current.entries)}`,
    ).toEqual(['Invoices', '2026'])
    expect(result.current.isComplete, 'a walked trail is complete').toBe(true)
})

/**
 * There is no ancestor-chain endpoint, so a cold deep link into a nested folder
 * can only name the folder it landed in.
 */
it('elides the ancestors it cannot know on a cold deep link', () => {
    const { result } = renderTrail(
        'dir-2',
        buildDriveDirectory({ id: 'dir-2', name: '2026', parentId: 'dir-1' }),
    )

    expect(
        result.current.entries,
        `expected only the current folder, got ${JSON.stringify(result.current.entries)}`,
    ).toEqual([{ id: 'dir-2', name: '2026' }])
    expect(
        result.current.isComplete,
        'the grandparents are unknown and must be shown as such',
    ).toBe(false)
})

it('knows the trail is whole when the folder sits at the root', () => {
    const { result } = renderTrail(
        'dir-1',
        buildDriveDirectory({ id: 'dir-1', name: 'Invoices', parentId: null }),
    )

    expect(
        result.current.isComplete,
        'a null parent proves there is nothing between the folder and Home',
    ).toBe(true)
})
