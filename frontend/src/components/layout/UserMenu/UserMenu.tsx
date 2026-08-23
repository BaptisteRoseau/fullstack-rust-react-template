import { Trans } from '@lingui/react/macro'
import { useNavigate } from 'react-router'

import type { CurrentUser } from '@/api/domains/currentUser'
import { useApiLogout } from '@/api/hooks/useApiLogout'
import { Avatar } from '@/design-system/Avatar'
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownLabel,
    DropdownSeparator,
    DropdownTrigger,
} from '@/design-system/Dropdown'
import { KeyIcon, LogoutIcon, UserIcon } from '@/design-system/Icon'
import { PATHS } from '@/router/constants'
import { fullName } from '@/utils/strings'

import styles from './user-menu.module.scss'

export type UserMenuProps = {
    user: CurrentUser
}

export function UserMenu({ user }: UserMenuProps) {
    const navigate = useNavigate()
    const { trigger: logout } = useApiLogout()

    const name = fullName(user)

    async function handleLogout() {
        await logout()
        await navigate(PATHS.home)
    }

    return (
        <Dropdown>
            <DropdownTrigger className={styles.trigger}>
                <Avatar name={name} aria-hidden />
                <span className={styles.name}>{name}</span>
            </DropdownTrigger>
            <DropdownContent>
                <DropdownLabel>{user.email}</DropdownLabel>
                <DropdownSeparator />
                <DropdownItem
                    onSelect={() => void navigate(PATHS.user.information)}
                >
                    <UserIcon />
                    <Trans>Information</Trans>
                </DropdownItem>
                <DropdownItem
                    onSelect={() => void navigate(PATHS.user.apiKeys)}
                >
                    <KeyIcon />
                    <Trans>API keys</Trans>
                </DropdownItem>
                <DropdownSeparator />
                <DropdownItem onSelect={() => void handleLogout()}>
                    <LogoutIcon />
                    <Trans>Log out</Trans>
                </DropdownItem>
            </DropdownContent>
        </Dropdown>
    )
}
