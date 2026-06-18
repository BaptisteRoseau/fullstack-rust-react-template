import { Trans } from '@lingui/macro'

import { Button } from '../ui/button'

export const MainErrorFallback = () => {
    return (
        <div
            className="flex h-screen w-screen flex-col items-center justify-center text-red-500"
            role="alert"
        >
            <h2 className="text-lg font-semibold">
                <Trans>Ooops, something went wrong :(</Trans>
            </h2>
            <Button
                className="mt-4"
                onClick={() => window.location.assign(window.location.origin)}
            >
                <Trans>Refresh</Trans>
            </Button>
        </div>
    )
}
