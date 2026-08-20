import { Link } from 'react-router'

import LogoSvg from '@/img/logo.svg?react'
import { PATHS } from '@/router/constants'

import styles from './logo.module.scss'

export type LogoProps = {
    label: string
}

export function Logo({ label }: LogoProps) {
    return (
        <Link to={PATHS.home} className={styles.logo} aria-label={label}>
            <LogoSvg className={styles.mark} width={32} height={32} />
            <span className={styles.name}>{label}</span>
        </Link>
    )
}
