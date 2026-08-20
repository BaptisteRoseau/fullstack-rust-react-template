import { useLingui } from '@lingui/react/macro'

import { SelectInput } from '@/design-system/inputs/SelectInput'
import { localeLabels, locales, type Locale } from '@/i18n'
import { useLocale } from '@/stores/locale'

import styles from './locale-switcher.module.scss'

export function LocaleSwitcher() {
    const { t } = useLingui()
    const locale = useLocale((state) => state.locale)
    const setLocale = useLocale((state) => state.setLocale)

    return (
        <SelectInput
            className={styles.select}
            aria-label={t`Language`}
            value={locale}
            onChange={(event) => void setLocale(event.target.value as Locale)}
            options={locales.map((value) => ({
                value,
                label: localeLabels[value],
            }))}
        />
    )
}
