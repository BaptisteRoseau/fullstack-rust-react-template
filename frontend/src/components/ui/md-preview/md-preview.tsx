import createDOMPurify from 'dompurify'
import { parse } from 'marked'

const DOMPurify = createDOMPurify(window)

export type MDPreviewProps = {
    value: string
}

export const MDPreview = ({ value = '' }: MDPreviewProps) => {
    return (
        <div
            className="prose w-full p-2 prose-slate"
            dangerouslySetInnerHTML={{
                __html: DOMPurify.sanitize(parse(value) as string),
            }}
        />
    )
}
