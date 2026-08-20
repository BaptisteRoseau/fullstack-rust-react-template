import { render as rtlRender } from '@testing-library/react'
import { createMemoryRouter, RouterProvider } from 'react-router'
import { SWRConfig } from 'swr'

import { Context } from '@/Context'
import { AppLayout } from '@/layouts/AppLayout'
import { Home } from '@/pages/Home'
import { Login } from '@/pages/Login'
import { NotFound } from '@/pages/NotFound'
import { PATHS } from '@/router/constants'

export function renderAppAtRoute(route: string) {
    const router = createMemoryRouter(
        [
            {
                element: <AppLayout />,
                children: [
                    { path: PATHS.home, element: <Home /> },
                    { path: PATHS.login, element: <Login /> },
                    { path: PATHS.notFound, element: <NotFound /> },
                ],
            },
        ],
        { initialEntries: [route] },
    )

    return rtlRender(
        <SWRConfig value={{ provider: () => new Map(), dedupingInterval: 0 }}>
            <Context>
                <RouterProvider router={router} />
            </Context>
        </SWRConfig>,
    )
}
