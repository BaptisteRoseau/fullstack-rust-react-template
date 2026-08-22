import { useCallback, useState } from 'react'

export function useCopyToClipboard(resetDelay = 2000) {
    const [isCopied, setIsCopied] = useState(false)

    const copy = useCallback(
        async (value: string) => {
            await navigator.clipboard.writeText(value)
            setIsCopied(true)
            window.setTimeout(() => setIsCopied(false), resetDelay)
        },
        [resetDelay],
    )

    return { isCopied, copy }
}
