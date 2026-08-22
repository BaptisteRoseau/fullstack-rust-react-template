import { useLingui } from '@lingui/react/macro'

import { Button } from '@/design-system/Button'
import {
    MonitorIcon,
    MoonIcon,
    SunIcon,
    type IconProps,
} from '@/design-system/Icon'
import { useTheme, type Theme } from '@/stores/theme'

import styles from './theme-switcher.module.scss'

const icons: Record<Theme, React.FC<IconProps>> = {
    system: MonitorIcon,
    light: SunIcon,
    dark: MoonIcon,
}

export function ThemeSwitcher() {
    const { t } = useLingui()
    const theme = useTheme((state) => state.theme)
    const cycleTheme = useTheme((state) => state.cycleTheme)

    const labels: Record<Theme, string> = {
        system: t`Theme: system. Switch to the light theme.`,
        light: t`Theme: light. Switch to the dark theme.`,
        dark: t`Theme: dark. Switch to the system theme.`,
    }
    const tooltips: Record<Theme, string> = {
        system: t`Switch to the light theme`,
        light: t`Switch to the dark theme`,
        dark: t`Switch to the system theme`,
    }
    const Icon = icons[theme]

    return (
        <Button
            variant="ghost"
            className={styles.button}
            aria-label={labels[theme]}
            title={tooltips[theme]}
            onClick={cycleTheme}
        >
            <Icon />
        </Button>
    )
}
