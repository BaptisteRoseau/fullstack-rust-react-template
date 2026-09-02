import { renderHook } from '@testing-library/react'

import { useObjectUrl } from './useObjectUrl'

/**
 * The object-URL factory is patched onto the real `URL` rather than replacing
 * it: jsdom leaves both methods undefined, and a wholesale stub would take the
 * constructor with it, which the fetch layer needs to parse request URLs.
 */
beforeEach(() => {
    URL.createObjectURL = vi.fn(() => 'blob:mock-url')
    URL.revokeObjectURL = vi.fn()
})

it('exposes a url for the blob it is given', () => {
    const { result } = renderHook(() => useObjectUrl(new Blob(['hello'])))

    expect(
        result.current,
        `expected a blob url, got ${String(result.current)}`,
    ).toBe('blob:mock-url')
})

it('answers null without a blob', () => {
    const { result } = renderHook(() => useObjectUrl(undefined))

    expect(
        result.current,
        `expected null, got ${String(result.current)}`,
    ).toBeNull()
})

it('revokes the url it created when it unmounts', () => {
    const { unmount } = renderHook(() => useObjectUrl(new Blob(['hello'])))

    unmount()

    expect(
        URL.revokeObjectURL,
        'an object url pins its blob in memory until it is revoked',
    ).toHaveBeenCalledWith('blob:mock-url')
})
