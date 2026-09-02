import { act, renderHook } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiDownloadFile } from './useApiDownloadFile'

/**
 * The object-URL factory is patched onto the real `URL` rather than replacing
 * it: jsdom leaves both methods undefined, and a wholesale stub would take the
 * constructor with it, which the fetch layer needs to parse request URLs.
 */
beforeEach(() => {
    URL.createObjectURL = vi.fn(() => 'blob:download')
    URL.revokeObjectURL = vi.fn()
})

it('saves the fetched content under the name it was given', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/download`), () =>
            HttpResponse.arrayBuffer(new Uint8Array([1, 2, 3]).buffer, {
                headers: { 'Content-Type': 'application/octet-stream' },
            }),
        ),
    )

    const clicks: { download: string; href: string }[] = []
    const click = vi
        .spyOn(HTMLAnchorElement.prototype, 'click')
        .mockImplementation(function (this: HTMLAnchorElement) {
            clicks.push({ download: this.download, href: this.href })
        })

    const { result } = renderHook(() => useApiDownloadFile(), {
        wrapper: SwrWrapper,
    })

    await act(() => result.current.download('file-42', 'report.pdf'))

    expect(
        clicks,
        `expected one save of report.pdf, got ${JSON.stringify(clicks)}`,
    ).toEqual([{ download: 'report.pdf', href: 'blob:download' }])
    expect(
        URL.revokeObjectURL,
        'the object url must be revoked once the click is dispatched',
    ).toHaveBeenCalledWith('blob:download')

    click.mockRestore()
})

it('clears its pending state once the save is done', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/download`), () =>
            HttpResponse.arrayBuffer(new ArrayBuffer(1), {
                headers: { 'Content-Type': 'application/octet-stream' },
            }),
        ),
    )
    vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {})

    const { result } = renderHook(() => useApiDownloadFile(), {
        wrapper: SwrWrapper,
    })

    await act(() => result.current.download('file-42', 'report.pdf'))

    expect(
        result.current.isDownloading,
        'the hook must not stay pending after a finished save',
    ).toBe(false)
})
