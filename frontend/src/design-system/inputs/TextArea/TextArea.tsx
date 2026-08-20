import clsx from 'clsx'

import styles from './text-area.module.scss'

export type TextAreaProps = React.TextareaHTMLAttributes<HTMLTextAreaElement>

export const TextArea = ({
    className,
    rows = 4,
    ref,
    ...props
}: TextAreaProps & { ref?: React.Ref<HTMLTextAreaElement> }) => (
    <textarea
        ref={ref}
        rows={rows}
        className={clsx(styles.textarea, className)}
        {...props}
    />
)
