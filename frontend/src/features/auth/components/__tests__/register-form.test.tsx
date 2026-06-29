import { renderApp, screen } from '@/testing/test-utils'

import { RegisterForm } from '../register-form'

test('shows a link that starts the Keycloak sign-up flow', async () => {
    await renderApp(<RegisterForm />, { user: null })

    const link = screen.getByRole('link', {
        name: /create an account/i,
    }) as HTMLAnchorElement

    expect(link.href).toContain('screen=register')
})
