import styles from './content-layout.module.scss'

export type ContentLayoutProps = {
    title: string
    description?: string
    actions?: React.ReactNode
    children: React.ReactNode
}

export function ContentLayout({
    title,
    description,
    actions,
    children,
}: ContentLayoutProps) {
    return (
        <section className={styles.layout}>
            <header className={styles.header}>
                <div>
                    <h1 className={styles.title}>{title}</h1>
                    {description ? (
                        <p className={styles.description}>{description}</p>
                    ) : null}
                </div>
                {actions ? (
                    <div className={styles.actions}>{actions}</div>
                ) : null}
            </header>
            {children}
        </section>
    )
}
