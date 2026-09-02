import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiFilePermissions } from '@/api/hooks/useApiFilePermissions'
import { buildGetPermissionResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiGrantFilePermission } from './useApiGrantFilePermission'

it('grants to the named user and refreshes the grants itself', async () => {
    let granted: { userId?: string; level?: unknown } = {}

    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/permissions`), () =>
            HttpResponse.json(
                granted.userId
                    ? [buildGetPermissionResponse({ grantee: granted.userId })]
                    : [],
            ),
        ),
        http.put(
            endpoint(`${API_PATHS.files}/:id/permissions/:userId`),
            async ({ params, request }) => {
                granted = {
                    userId: String(params.userId),
                    level: ((await request.json()) as { level: string }).level,
                }
                return HttpResponse.json(buildGetPermissionResponse())
            },
        ),
    )

    const { result } = renderHook(
        () => ({
            grants: useApiFilePermissions('file-42'),
            grant: useApiGrantFilePermission('file-42'),
        }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.grants.data?.length,
            'the grants should have loaded first',
        ).toBe(0),
    )

    await act(() =>
        result.current.grant.trigger({ userId: 'user-7', level: 'editor' }),
    )

    expect(
        granted,
        `expected user-7 as an editor, got ${JSON.stringify(granted)}`,
    ).toEqual({ userId: 'user-7', level: 'editor' })
    await waitFor(() =>
        expect(
            result.current.grants.data?.length,
            `the mutation must invalidate the grants itself, got ${result.current.grants.data?.length}`,
        ).toBe(1),
    )
})
