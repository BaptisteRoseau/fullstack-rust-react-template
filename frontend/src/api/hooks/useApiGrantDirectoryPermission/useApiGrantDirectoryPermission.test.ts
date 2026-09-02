import { act, renderHook } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetPermissionResponse } from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiGrantDirectoryPermission } from './useApiGrantDirectoryPermission'

it('grants the level on the directory the hook was given', async () => {
    let target: { directoryId?: string; userId?: string } = {}

    server.use(
        http.put(
            endpoint(`${API_PATHS.directories}/:id/permissions/:userId`),
            ({ params }) => {
                target = {
                    directoryId: String(params.id),
                    userId: String(params.userId),
                }
                return HttpResponse.json(
                    buildGetPermissionResponse({ level: 'manager' }),
                )
            },
        ),
    )

    const { result } = renderHook(
        () => useApiGrantDirectoryPermission('dir-42'),
        { wrapper: SwrWrapper },
    )

    const grant = await act(() =>
        result.current.trigger({ userId: 'user-7', level: 'manager' }),
    )

    expect(
        target,
        `expected dir-42 and user-7, got ${JSON.stringify(target)}`,
    ).toEqual({ directoryId: 'dir-42', userId: 'user-7' })
    expect(grant.level, `expected "manager", got "${grant.level}"`).toBe(
        'manager',
    )
})
