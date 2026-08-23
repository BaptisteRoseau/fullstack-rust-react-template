import { Trans, useLingui } from '@lingui/react/macro'

import type { ApiKey } from '@/api/domains/apiKeys'
import { Badge } from '@/design-system/Badge'
import { Card } from '@/design-system/Card'
import { Spinner } from '@/design-system/Spinner'
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from '@/design-system/Table'
import { formatDate } from '@/utils/date'

import { RevokeApiKeyButton } from '../RevokeApiKeyButton'

import styles from './api-keys-table.module.scss'

export type ApiKeysTableProps = {
    apiKeys: ApiKey[]
    isLoading: boolean
    error: unknown
}

export function ApiKeysTable({ apiKeys, isLoading, error }: ApiKeysTableProps) {
    const { t } = useLingui()

    if (isLoading) {
        return (
            <Card className={styles.state}>
                <Spinner label={t`Loading`} />
            </Card>
        )
    }

    if (error) {
        return (
            <Card className={styles.state} role="alert">
                <Trans>The API keys could not be loaded.</Trans>
            </Card>
        )
    }

    if (apiKeys.length === 0) {
        return (
            <Card className={styles.state}>
                <Trans>You have no API key yet.</Trans>
            </Card>
        )
    }

    return (
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>
                        <Trans>Name</Trans>
                    </TableHead>
                    <TableHead>
                        <Trans>Permissions</Trans>
                    </TableHead>
                    <TableHead>
                        <Trans>Created</Trans>
                    </TableHead>
                    <TableHead>
                        <span className={styles.visuallyHidden}>
                            <Trans>Actions</Trans>
                        </span>
                    </TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {apiKeys.map((apiKey) => (
                    <TableRow key={apiKey.id}>
                        <TableCell>{apiKey.name}</TableCell>
                        <TableCell>
                            <span className={styles.permissions}>
                                {apiKey.permissions.map((permission) => (
                                    <Badge key={permission}>{permission}</Badge>
                                ))}
                            </span>
                        </TableCell>
                        <TableCell>{formatDate(apiKey.createdAt)}</TableCell>
                        <TableCell className={styles.actions}>
                            <RevokeApiKeyButton apiKey={apiKey} />
                        </TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    )
}
