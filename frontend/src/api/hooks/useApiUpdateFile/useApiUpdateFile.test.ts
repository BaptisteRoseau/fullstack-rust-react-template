import { act, renderHook } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetFileResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiUpdateFile } from './useApiUpdateFile'

it('moves the file to the destination it is given', async () => {
    let body: unknown = null

    server.use(
        http.patch(endpoint(`${API_PATHS.files}/:id`), async ({ request }) => {
            body = await request.json()
            return HttpResponse.json(buildGetFileResponse())
        }),
    )

    const { result } = renderHook(() => useApiUpdateFile('file-42'), {
        wrapper: SwrWrapper,
    })

    await act(() => result.current.trigger({ parentId: null }))

    expect(
        body,
        `a move to the root must send an explicit null parent, got ${JSON.stringify(body)}`,
    ).toEqual({ parentId: null })
})
