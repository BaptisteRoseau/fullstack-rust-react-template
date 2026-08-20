import { createBrowserRouter, Outlet } from 'react-router'

import { ProtectedRoute } from '@/components/ProtectedRoute'
import { AppLayout } from '@/layouts/AppLayout'
import { AuthLayout } from '@/layouts/AuthLayout'

import { PATHS } from './constants'

export const router = createBrowserRouter([
    {
        HydrateFallback: () => null,
        element: <AuthLayout />,
        children: [
            {
                path: PATHS.login,
                lazy: async () => ({
                    Component: (await import('@/pages/Login')).Login,
                }),
            },
            {
                path: PATHS.register,
                lazy: async () => ({
                    Component: (await import('@/pages/Register')).Register,
                }),
            },
        ],
    },
    {
        HydrateFallback: () => null,
        element: <AppLayout />,
        children: [
            {
                path: PATHS.home,
                lazy: async () => ({
                    Component: (await import('@/pages/Home')).Home,
                }),
            },
            {
                element: (
                    <ProtectedRoute>
                        <Outlet />
                    </ProtectedRoute>
                ),
                children: [
                    {
                        path: PATHS.user.root,
                        lazy: async () => ({
                            Component: (await import('@/pages/User')).User,
                        }),
                        children: [
                            {
                                index: true,
                                lazy: async () => ({
                                    Component: (await import('@/pages/User'))
                                        .Information,
                                }),
                            },
                            {
                                path: PATHS.user.apiKeys,
                                lazy: async () => ({
                                    Component: (await import('@/pages/User'))
                                        .ApiKeys,
                                }),
                            },
                        ],
                    },
                ],
            },
            {
                path: PATHS.notFound,
                lazy: async () => ({
                    Component: (await import('@/pages/NotFound')).NotFound,
                }),
            },
        ],
    },
])
