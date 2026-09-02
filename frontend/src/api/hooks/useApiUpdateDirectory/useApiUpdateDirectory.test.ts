import { act, renderHook } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetDirectoryResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiUpdateDirectory } from './useApiUpdateDirectory'

it('patches the directory the hook was given', async () => {
    let patchedId: string | null = null

    server.use(
        http.patch(endpoint(`${API_PATHS.directories}/:id`), ({ params }) => {
            patchedId = String(params.id)
            return HttpResponse.json(
                buildGetDirectoryResponse({ name: 'Archive' }),
            )
        }),
    )

    const { result } = renderHook(() => useApiUpdateDirectory('dir-42'), {
        wrapper: SwrWrapper,
    })

    const updated = await act(() => result.current.trigger({ name: 'Archive' }))

    expect(
        patchedId,
        `the patch must target the directory the hook was given, got ${String(patchedId)}`,
    ).toBe('dir-42')
    expect(updated.name, `expected "Archive", got "${updated.name}"`).toBe(
        'Archive',
    )
})
