import { QueryClient, useQueryClient } from '@tanstack/react-query'
import { useMemo } from 'react'
import { createBrowserRouter } from 'react-router'
import { RouterProvider } from 'react-router/dom'

import { paths } from '@/config/paths'
import { ProtectedRoute } from '@/lib/auth'

import {
    default as AppRoot,
    ErrorBoundary as AppRootErrorBoundary,
} from './pages/app/root'

const convert = (queryClient: QueryClient) => (m: any) => {
    const { clientLoader, clientAction, default: Component, ...rest } = m
    return {
        ...rest,
        loader: clientLoader?.(queryClient),
        action: clientAction?.(queryClient),
        Component,
    }
}

export const createAppRouter = (queryClient: QueryClient) =>
    createBrowserRouter([
        {
            path: paths.home.path,
            lazy: () => import('./pages/landing').then(convert(queryClient)),
        },
        {
            path: paths.app.root.path,
            element: (
                <ProtectedRoute>
                    <AppRoot />
                </ProtectedRoute>
            ),
            ErrorBoundary: AppRootErrorBoundary,
            children: [
                {
                    path: paths.app.discussions.path,
                    lazy: () =>
                        import('./pages/app/discussions/discussions').then(
                            convert(queryClient),
                        ),
                },
                {
                    path: paths.app.discussion.path,
                    lazy: () =>
                        import('./pages/app/discussions/discussion').then(
                            convert(queryClient),
                        ),
                },
                {
                    path: paths.app.users.path,
                    lazy: () =>
                        import('./pages/app/users').then(convert(queryClient)),
                },
                {
                    path: paths.app.profile.path,
                    lazy: () =>
                        import('./pages/app/profile').then(
                            convert(queryClient),
                        ),
                },
                {
                    path: paths.app.dashboard.path,
                    lazy: () =>
                        import('./pages/app/dashboard').then(
                            convert(queryClient),
                        ),
                },
            ],
        },
        {
            path: '*',
            lazy: () => import('./pages/not-found').then(convert(queryClient)),
        },
    ])

export const AppRouter = () => {
    const queryClient = useQueryClient()

    const router = useMemo(() => createAppRouter(queryClient), [queryClient])

    return <RouterProvider router={router} />
}
