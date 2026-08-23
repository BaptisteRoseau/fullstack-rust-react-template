import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiApiKeys } from '@/api/hooks/useApiApiKeys'
import { buildGetApiKeyResponse } from '@/test-utils/fixtures/apiKeys'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiRevokeApiKey } from './useApiRevokeApiKey'

it('deletes the key and refreshes the list itself', async () => {
    let revoked = false

    server.use(
        http.get(endpoint(API_PATHS.apiKeys), () =>
            HttpResponse.json(
                revoked ? [] : [buildGetApiKeyResponse({ id: 'key-42' })],
            ),
        ),
        http.delete(endpoint(`${API_PATHS.apiKeys}/:id`), ({ params }) => {
            revoked = String(params.id) === 'key-42'
            return HttpResponse.text(null, { status: 204 })
        }),
    )

    const { result } = renderHook(
        () => ({
            list: useApiApiKeys(),
            revoke: useApiRevokeApiKey('key-42'),
        }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.length,
            'the list should have loaded first',
        ).toBe(1),
    )

    await act(() => result.current.revoke.trigger())

    expect(revoked, 'the delete must target the key the hook was given').toBe(
        true,
    )
    await waitFor(() =>
        expect(
            result.current.list.data?.length,
            `the mutation must invalidate the list itself, got ${result.current.list.data?.length}`,
        ).toBe(0),
    )
})
