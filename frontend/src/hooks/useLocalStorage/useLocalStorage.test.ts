import { act, renderHook } from '@testing-library/react'

import { useLocalStorage } from './useLocalStorage'

afterEach(() => {
    // `setup-tests.ts` resets the mock database and mocks, but not jsdom's
    // localStorage, which would otherwise leak between tests.
    window.localStorage.clear()
})

it('falls back to the initial value when nothing is stored', () => {
    const { result } = renderHook(() => useLocalStorage('sidebar', 'expanded'))
    const [value] = result.current

    expect(
        value,
        `expected the initial "expanded" for an empty store, got ${JSON.stringify(value)}`,
    ).toBe('expanded')
    expect(
        window.localStorage.getItem('sidebar'),
        `expected reading not to write anything, got ${window.localStorage.getItem('sidebar')}`,
    ).toBeNull()
})

it('restores the stored value over the initial one', () => {
    window.localStorage.setItem('sidebar', JSON.stringify('collapsed'))

    const { result } = renderHook(() => useLocalStorage('sidebar', 'expanded'))

    expect(
        result.current[0],
        `expected the stored "collapsed", got ${JSON.stringify(result.current[0])}`,
    ).toBe('collapsed')
})

it('persists a new value and exposes it immediately', () => {
    const { result } = renderHook(() => useLocalStorage('sidebar', 'expanded'))

    act(() => result.current[1]('collapsed'))

    expect(
        result.current[0],
        `expected the hook to expose "collapsed", got ${JSON.stringify(result.current[0])}`,
    ).toBe('collapsed')
    expect(
        window.localStorage.getItem('sidebar'),
        `expected "collapsed" serialised in localStorage, got ${window.localStorage.getItem('sidebar')}`,
    ).toBe(JSON.stringify('collapsed'))
})

it('round-trips non-string values through JSON', () => {
    const preferences = { density: 'compact', pageSize: 25, pinned: ['name'] }
    const { result } = renderHook(() =>
        useLocalStorage('preferences', {
            density: 'cosy',
            pageSize: 10,
            pinned: [] as string[],
        }),
    )

    act(() => result.current[1](preferences))
    const { result: remounted } = renderHook(() =>
        useLocalStorage('preferences', {
            density: 'cosy',
            pageSize: 10,
            pinned: [] as string[],
        }),
    )

    expect(
        remounted.current[0],
        `expected the object to survive a remount, got ${JSON.stringify(remounted.current[0])}`,
    ).toEqual(preferences)
})

it('reads and writes falsy values instead of treating them as absent', () => {
    window.localStorage.setItem('onboarded', JSON.stringify(false))

    const { result } = renderHook(() => useLocalStorage('onboarded', true))

    expect(
        result.current[0],
        `expected the stored false to win over the initial true, got ${result.current[0]}`,
    ).toBe(false)
})

it('keeps separate keys independent', () => {
    const { result: sidebar } = renderHook(() =>
        useLocalStorage('sidebar', 'expanded'),
    )
    const { result: theme } = renderHook(() =>
        useLocalStorage('theme', 'light'),
    )

    act(() => sidebar.current[1]('collapsed'))

    expect(
        theme.current[0],
        `expected the theme key to be untouched, got ${JSON.stringify(theme.current[0])}`,
    ).toBe('light')
    expect(
        window.localStorage.getItem('theme'),
        `expected nothing written under "theme", got ${window.localStorage.getItem('theme')}`,
    ).toBeNull()
})

it('keeps the setter stable across renders', () => {
    const { result, rerender } = renderHook(() =>
        useLocalStorage('sidebar', 'expanded'),
    )
    const [, store] = result.current

    act(() => result.current[1]('collapsed'))
    rerender()

    expect(
        result.current[1] === store,
        'expected the setter to keep its identity across renders, so consumers can put it in dependency arrays',
    ).toBe(true)
})
