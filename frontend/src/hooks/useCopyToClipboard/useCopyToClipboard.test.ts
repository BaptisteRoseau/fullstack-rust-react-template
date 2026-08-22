import { act, renderHook } from '@testing-library/react'

import { useCopyToClipboard } from './useCopyToClipboard'

const writeText = vi.fn<(value: string) => Promise<void>>()

beforeEach(() => {
    vi.useFakeTimers()
    writeText.mockResolvedValue(undefined)
    vi.stubGlobal('navigator', { clipboard: { writeText } })
})

afterEach(() => {
    vi.useRealTimers()
    vi.unstubAllGlobals()
})

it('starts out having copied nothing', () => {
    const { result } = renderHook(() => useCopyToClipboard())

    expect(
        result.current.isCopied,
        `expected isCopied false before any copy, got ${result.current.isCopied}`,
    ).toBe(false)
})

it('writes to the clipboard and flags the copy until the reset delay elapses', async () => {
    const { result } = renderHook(() => useCopyToClipboard(2000))

    await act(() => result.current.copy('secret-token'))

    expect(
        writeText.mock.calls,
        `expected a single write of "secret-token", got ${JSON.stringify(writeText.mock.calls)}`,
    ).toEqual([['secret-token']])
    expect(
        result.current.isCopied,
        `expected isCopied true right after copying, got ${result.current.isCopied}`,
    ).toBe(true)

    act(() => void vi.advanceTimersByTime(1999))
    expect(
        result.current.isCopied,
        `expected isCopied to still be true 1ms before the delay, got ${result.current.isCopied}`,
    ).toBe(true)

    act(() => void vi.advanceTimersByTime(1))
    expect(
        result.current.isCopied,
        `expected isCopied false once the 2000ms delay elapsed, got ${result.current.isCopied}`,
    ).toBe(false)
})

it('honours a custom reset delay', async () => {
    const { result } = renderHook(() => useCopyToClipboard(50))

    await act(() => result.current.copy('short-lived'))
    act(() => void vi.advanceTimersByTime(50))

    expect(
        result.current.isCopied,
        `expected the custom 50ms delay to clear isCopied, got ${result.current.isCopied}`,
    ).toBe(false)
})

it('leaves the copied flag down when the clipboard write is refused', async () => {
    writeText.mockRejectedValue(new Error('permission denied'))
    const { result } = renderHook(() => useCopyToClipboard())

    await expect(
        act(() => result.current.copy('secret-token')),
        'expected the rejection to surface to the caller instead of being swallowed',
    ).rejects.toThrow('permission denied')

    expect(
        result.current.isCopied,
        `expected isCopied false after a failed write, got ${result.current.isCopied}`,
    ).toBe(false)
})
