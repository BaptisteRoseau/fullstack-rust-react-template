import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetPermissionResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiDirectoryPermissions } from './useApiDirectoryPermissions'

it('returns the grants on the directory named by its cache key', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.directories}/:id/permissions`), () =>
            HttpResponse.json([
                buildGetPermissionResponse({ grantee: 'user-7' }),
            ]),
        ),
    )

    const { result } = renderHook(() => useApiDirectoryPermissions('dir-42'), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.[0].grantee,
            `expected "user-7", got ${result.current.data?.[0].grantee} (error: ${result.current.error})`,
        ).toBe('user-7'),
    )
})
