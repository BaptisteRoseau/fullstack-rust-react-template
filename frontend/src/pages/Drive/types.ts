/** What a per-entry control acts on: the two halves of a listing look alike. */
export type DriveEntryKind = 'directory' | 'file'

/** One step of the breadcrumb trail, accumulated as the user navigates in. */
export type DriveTrailEntry = {
    id: string
    name: string
}
