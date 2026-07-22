import { env } from '@/config/env'
import { renderApp, screen } from '@/testing/test-utils'

import { RegisterForm } from '../register-form'

test('shows a link that starts the Keycloak sign-up flow', async () => {
    await renderApp(<RegisterForm />, { user: null })

    const link = screen.getByRole('link', {
        name: /create an account/i,
    }) as HTMLAnchorElement

    // Registration is its own entrypoint on the auth BFF, which drives the
    // redirect to Keycloak's hosted sign-up page.
    expect(link.href).toBe(`${env.API_URL}/api/auth/register`)
})
