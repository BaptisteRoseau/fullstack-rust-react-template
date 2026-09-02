export const PATHS = {
    home: '/',
    login: '/auth/login',
    register: '/auth/register',
    drive: {
        root: '/drive',
        /** A route pattern, not a URL — build one with {@link driveDirectoryPath}. */
        directory: '/drive/:directoryId',
    },
    legal: {
        terms: '/legal/terms',
        mentions: '/legal/mentions',
    },
    user: {
        root: '/user',
        information: '/user',
        apiKeys: '/user/api-keys',
    },
    notFound: '*',
} as const

/**
 * The one place a drive URL is assembled: `PATHS.drive.directory` is the route
 * pattern the router matches, never something a link may use as-is.
 */
export function driveDirectoryPath(directoryId: string): string {
    return PATHS.drive.directory.replace(':directoryId', directoryId)
}
