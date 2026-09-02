import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiThumbnail } from './useApiThumbnail'

/**
 * The object-URL factory is patched onto the real `URL` rather than replacing
 * it: jsdom leaves both methods undefined, and a wholesale stub would take the
 * constructor with it, which the fetch layer needs to parse request URLs.
 */
beforeEach(() => {
    URL.createObjectURL = vi.fn(() => 'blob:thumbnail')
    URL.revokeObjectURL = vi.fn()
})

it('exposes the fetched thumbnail as an object url', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/thumbnail`), () =>
            HttpResponse.arrayBuffer(new Uint8Array([1, 2, 3]).buffer, {
                headers: { 'Content-Type': 'image/webp' },
            }),
        ),
    )

    const { result } = renderHook(() => useApiThumbnail('file-42'), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.url,
            `expected an object url, got ${String(result.current.url)} (error: ${result.current.error})`,
        ).toBe('blob:thumbnail'),
    )
})

it('skips the request entirely without a file', async () => {
    let requests = 0

    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/thumbnail`), () => {
            requests += 1
            return HttpResponse.arrayBuffer(new ArrayBuffer(0))
        }),
    )

    renderHook(() => useApiThumbnail(undefined), { wrapper: SwrWrapper })

    await waitFor(() =>
        expect(requests, `expected no request, got ${requests}`).toBe(0),
    )
})
