import {
    createContext as reactCreateContext,
    useContext as reactUseContext,
} from 'react'

export function createContext<T>(displayName: string) {
    const Context = reactCreateContext<T | undefined>(undefined)
    Context.displayName = displayName

    function useContext(): T {
        const value = reactUseContext(Context)
        if (value === undefined) {
            throw new Error(
                `use${displayName} must be used inside <${displayName}Provider>`,
            )
        }
        return value
    }

    return [Context.Provider, useContext] as const
}
