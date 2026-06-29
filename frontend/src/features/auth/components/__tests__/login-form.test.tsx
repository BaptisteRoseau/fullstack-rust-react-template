import { renderApp, screen } from '@/testing/test-utils'

import { LoginForm } from '../login-form'

test('shows a link that starts the Keycloak sign-in flow', async () => {
    await renderApp(<LoginForm />, { user: null })

    const link = screen.getByRole('link', {
        name: /continue to sign in/i,
    }) as HTMLAnchorElement

    expect(link.href).toContain('/auth/login')
})
