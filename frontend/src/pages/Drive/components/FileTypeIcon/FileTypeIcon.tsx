import {
    FileIcon,
    FileTextIcon,
    type IconProps,
    ImageIcon,
} from '@/design-system/Icon'
import { isPdf, mimeTypeGroup } from '@/utils/files'

export type FileTypeIconProps = IconProps & {
    mimeType: string
}

/** The closest icon the set has to a MIME type; a plain sheet otherwise. */
export function FileTypeIcon({ mimeType, ...props }: FileTypeIconProps) {
    if (mimeTypeGroup(mimeType) === 'image') {
        return <ImageIcon {...props} />
    }
    if (mimeTypeGroup(mimeType) === 'text' || isPdf(mimeType)) {
        return <FileTextIcon {...props} />
    }
    return <FileIcon {...props} />
}
