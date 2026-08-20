import { Trans } from '@lingui/react/macro'

import { Button } from '@/design-system/Button'
import {
    DialogClose,
    DialogContent,
    DialogFooter,
    DialogRoot,
    DialogTrigger,
} from '@/design-system/Dialog'

export type ConfirmationDialogProps = {
    title: string
    description: string
    confirmLabel: string
    trigger: React.ReactNode
    isConfirming?: boolean
    onConfirm: () => void
}

export function ConfirmationDialog({
    title,
    description,
    confirmLabel,
    trigger,
    isConfirming = false,
    onConfirm,
}: ConfirmationDialogProps) {
    return (
        <DialogRoot>
            <DialogTrigger asChild>{trigger}</DialogTrigger>
            <DialogContent title={title} description={description}>
                <DialogFooter>
                    <DialogClose asChild>
                        <Button variant="secondary">
                            <Trans>Cancel</Trans>
                        </Button>
                    </DialogClose>
                    <Button
                        variant="danger"
                        disabled={isConfirming}
                        onClick={onConfirm}
                    >
                        {confirmLabel}
                    </Button>
                </DialogFooter>
            </DialogContent>
        </DialogRoot>
    )
}
