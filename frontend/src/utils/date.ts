import dayjs from 'dayjs'

import { DATE_FORMAT, DATETIME_FORMAT } from '@/constants/dates'

export function formatDate(value: Date | string | number): string {
    return dayjs(value).format(DATE_FORMAT)
}

export function formatDateTime(value: Date | string | number): string {
    return dayjs(value).format(DATETIME_FORMAT)
}
