import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetPermissionResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiFilePermissions } from './useApiFilePermissions'

it('returns the grants on the file named by its cache key', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/permissions`), () =>
            HttpResponse.json([
                buildGetPermissionResponse({ level: 'editor' }),
            ]),
        ),
    )

    const { result } = renderHook(() => useApiFilePermissions('file-42'), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.[0].level,
            `expected "editor", got ${result.current.data?.[0].level} (error: ${result.current.error})`,
        ).toBe('editor'),
    )
})

it('skips the request entirely without a file', async () => {
    let requests = 0

    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/permissions`), () => {
            requests += 1
            return HttpResponse.json([])
        }),
    )

    const { result } = renderHook(() => useApiFilePermissions(undefined), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(result.current.isLoading, 'the hook should have settled').toBe(
            false,
        ),
    )
    expect(requests, `expected no request, got ${requests}`).toBe(0)
})
