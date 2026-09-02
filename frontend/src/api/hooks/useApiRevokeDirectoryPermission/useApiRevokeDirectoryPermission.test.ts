import { act, renderHook } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiRevokeDirectoryPermission } from './useApiRevokeDirectoryPermission'

it('revokes the named user on the directory the hook was given', async () => {
    let target: { directoryId?: string; userId?: string } = {}

    server.use(
        http.delete(
            endpoint(`${API_PATHS.directories}/:id/permissions/:userId`),
            ({ params }) => {
                target = {
                    directoryId: String(params.id),
                    userId: String(params.userId),
                }
                return HttpResponse.text(null, { status: 204 })
            },
        ),
    )

    const { result } = renderHook(
        () => useApiRevokeDirectoryPermission('dir-42'),
        { wrapper: SwrWrapper },
    )

    await act(() => result.current.trigger('user-7'))

    expect(
        target,
        `expected dir-42 and user-7, got ${JSON.stringify(target)}`,
    ).toEqual({ directoryId: 'dir-42', userId: 'user-7' })
})
