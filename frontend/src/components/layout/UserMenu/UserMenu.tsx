import { Trans } from '@lingui/react/macro'
import { useNavigate } from 'react-router'
import { useSWRConfig } from 'swr'

import { fullName, ME_ENDPOINT, type CurrentUser } from '@/api/auth'
import { useLogout } from '@/api/service/auth'
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

import styles from './user-menu.module.scss'

export type UserMenuProps = {
    user: CurrentUser
}

export function UserMenu({ user }: UserMenuProps) {
    const navigate = useNavigate()
    const { mutate } = useSWRConfig()
    const { trigger: logout } = useLogout()

    const name = fullName(user)

    async function handleLogout() {
        await logout()
        await mutate(ME_ENDPOINT, null, { revalidate: false })
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
