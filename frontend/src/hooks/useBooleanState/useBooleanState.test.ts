import { act, renderHook } from '@testing-library/react'

import { useBooleanState } from './useBooleanState'

it('defaults to false and to the given initial value', () => {
    const { result: withDefault } = renderHook(() => useBooleanState())
    const { result: withInitial } = renderHook(() => useBooleanState(true))

    expect(
        withDefault.current.value,
        `expected false without an initial value, got ${withDefault.current.value}`,
    ).toBe(false)
    expect(
        withInitial.current.value,
        `expected the initial true, got ${withInitial.current.value}`,
    ).toBe(true)
})

it('sets, clears and toggles the value', () => {
    const { result } = renderHook(() => useBooleanState())

    act(() => result.current.setTrue())
    expect(
        result.current.value,
        `expected true after setTrue, got ${result.current.value}`,
    ).toBe(true)

    act(() => result.current.setFalse())
    expect(
        result.current.value,
        `expected false after setFalse, got ${result.current.value}`,
    ).toBe(false)

    act(() => result.current.toggle())
    expect(
        result.current.value,
        `expected true after toggling from false, got ${result.current.value}`,
    ).toBe(true)

    act(() => result.current.toggle())
    expect(
        result.current.value,
        `expected false after toggling from true, got ${result.current.value}`,
    ).toBe(false)
})

it('keeps the callbacks stable across renders', () => {
    const { result, rerender } = renderHook(() => useBooleanState())
    const { setTrue, setFalse, toggle } = result.current

    act(() => result.current.setTrue())
    rerender()

    expect(
        result.current.setTrue === setTrue &&
            result.current.setFalse === setFalse &&
            result.current.toggle === toggle,
        'expected the setters to keep their identity across renders, so consumers can put them in dependency arrays',
    ).toBe(true)
})
