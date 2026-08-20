export const PATHS = {
    home: '/',
    login: '/auth/login',
    register: '/auth/register',
    user: {
        root: '/user',
        information: '/user',
        apiKeys: '/user/api-keys',
    },
    notFound: '*',
} as const
