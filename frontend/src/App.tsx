import { RouterProvider } from 'react-router'

import { Context } from './Context'
import { router } from './router/routes'

export function App() {
    return (
        <Context>
            <RouterProvider router={router} />
        </Context>
    )
}
