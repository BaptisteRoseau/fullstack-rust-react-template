import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiFileContent } from './useApiFileContent'

it('returns the file content as a blob', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/download`), () =>
            HttpResponse.arrayBuffer(new Uint8Array([1, 2, 3]).buffer, {
                headers: { 'Content-Type': 'application/octet-stream' },
            }),
        ),
    )

    const { result } = renderHook(() => useApiFileContent('file-42'), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.size,
            `expected 3 bytes, got ${result.current.data?.size} (error: ${result.current.error})`,
        ).toBe(3),
    )
})

it('skips the request entirely without a file', async () => {
    let requests = 0

    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/download`), () => {
            requests += 1
            return HttpResponse.arrayBuffer(new ArrayBuffer(0))
        }),
    )

    const { result } = renderHook(() => useApiFileContent(undefined), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(result.current.isLoading, 'the hook should have settled').toBe(
            false,
        ),
    )
    expect(requests, `expected no request, got ${requests}`).toBe(0)
})
