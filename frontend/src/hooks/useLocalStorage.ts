import { useCallback, useState } from 'react'

export function useLocalStorage<T>(key: string, initialValue: T) {
    const [value, setValue] = useState<T>(() => {
        const stored = window.localStorage.getItem(key)
        return stored === null ? initialValue : (JSON.parse(stored) as T)
    })

    const store = useCallback(
        (next: T) => {
            setValue(next)
            window.localStorage.setItem(key, JSON.stringify(next))
        },
        [key],
    )

    return [value, store] as const
}
