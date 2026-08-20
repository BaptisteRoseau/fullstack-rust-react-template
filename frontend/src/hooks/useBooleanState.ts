import { useCallback, useState } from 'react'

export function useBooleanState(initialValue = false) {
    const [value, setValue] = useState(initialValue)

    const setTrue = useCallback(() => setValue(true), [])
    const setFalse = useCallback(() => setValue(false), [])
    const toggle = useCallback(() => setValue((current) => !current), [])

    return { value, setTrue, setFalse, toggle }
}
