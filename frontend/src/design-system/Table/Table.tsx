import clsx from 'clsx'

import styles from './table.module.scss'

export function Table({
    className,
    ...props
}: React.TableHTMLAttributes<HTMLTableElement>) {
    return (
        <div className={styles.wrapper}>
            <table className={clsx(styles.table, className)} {...props} />
        </div>
    )
}

export function TableHeader(
    props: React.HTMLAttributes<HTMLTableSectionElement>,
) {
    return <thead className={styles.header} {...props} />
}

export function TableBody(
    props: React.HTMLAttributes<HTMLTableSectionElement>,
) {
    return <tbody {...props} />
}

export function TableRow(props: React.HTMLAttributes<HTMLTableRowElement>) {
    return <tr className={styles.row} {...props} />
}

export function TableHead(props: React.ThHTMLAttributes<HTMLTableCellElement>) {
    return <th scope="col" className={styles.cell} {...props} />
}

export function TableCell(props: React.TdHTMLAttributes<HTMLTableCellElement>) {
    return <td className={styles.cell} {...props} />
}
