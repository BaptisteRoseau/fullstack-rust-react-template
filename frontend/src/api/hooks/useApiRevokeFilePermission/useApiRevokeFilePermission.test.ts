import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiFilePermissions } from '@/api/hooks/useApiFilePermissions'
import { buildGetPermissionResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiRevokeFilePermission } from './useApiRevokeFilePermission'

it('revokes the named user and refreshes the grants itself', async () => {
    let revokedUserId: string | null = null

    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/permissions`), () =>
            HttpResponse.json(
                revokedUserId
                    ? []
                    : [buildGetPermissionResponse({ grantee: 'user-7' })],
            ),
        ),
        http.delete(
            endpoint(`${API_PATHS.files}/:id/permissions/:userId`),
            ({ params }) => {
                revokedUserId = String(params.userId)
                return HttpResponse.text(null, { status: 204 })
            },
        ),
    )

    const { result } = renderHook(
        () => ({
            grants: useApiFilePermissions('file-42'),
            revoke: useApiRevokeFilePermission('file-42'),
        }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.grants.data?.length,
            'the grants should have loaded first',
        ).toBe(1),
    )

    await act(() => result.current.revoke.trigger('user-7'))

    expect(
        revokedUserId,
        `the revoke must target the user it was given, got ${String(revokedUserId)}`,
    ).toBe('user-7')
    await waitFor(() =>
        expect(
            result.current.grants.data?.length,
            `the mutation must invalidate the grants itself, got ${result.current.grants.data?.length}`,
        ).toBe(0),
    )
})
