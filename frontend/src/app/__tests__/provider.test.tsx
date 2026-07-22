import { HttpResponse, http } from 'msw'

import { env } from '@/config/env'
import { server } from '@/testing/mocks/server'
import { renderApp, screen } from '@/testing/test-utils'

// A failing `/api/auth/me` must never take the whole app down: the site keeps
// rendering its public view and the failure is reported through a notification.
const publicContent = 'Public content'

test('renders the app when the auth request fails with a server error', async () => {
    server.use(
        http.get(`${env.API_URL}/api/auth/me`, () =>
            HttpResponse.json(
                { id: 'UNEXPECTED', error: 'An unexpected error occurred.' },
                { status: 500 },
            ),
        ),
    )

    await renderApp(<div>{publicContent}</div>, { user: null })

    expect(await screen.findByText(publicContent)).toBeInTheDocument()
    expect(
        await screen.findByText(/an unexpected error occurred/i),
    ).toBeInTheDocument()
})

test('renders the app when the auth request fails with a network error', async () => {
    server.use(
        http.get(`${env.API_URL}/api/auth/me`, () => HttpResponse.error()),
    )

    await renderApp(<div>{publicContent}</div>, { user: null })

    expect(await screen.findByText(publicContent)).toBeInTheDocument()
})

test('renders the app unauthenticated when the user is not logged in', async () => {
    await renderApp(<div>{publicContent}</div>, { user: null })

    expect(await screen.findByText(publicContent)).toBeInTheDocument()
    // A logged-out visitor is an expected state, not an error worth a toast.
    expect(screen.queryByRole('alert')).not.toBeInTheDocument()
})
