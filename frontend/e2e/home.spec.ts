import { expect, test } from './utils/fixtures'
import { accountNav, expectSignedOut, login } from './utils/session'

test('shows the hero, the features and the footer', async ({ page }) => {
    await page.goto('/')

    await expect(
        page.getByRole('heading', {
            name: 'Ship a fullstack app, not a toolchain',
        }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'Authentication built in' }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'A typed API layer' }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'A real design system' }),
    ).toBeVisible()
    await expect(page.getByRole('contentinfo')).toBeVisible()
    await expectSignedOut(page)
})

test('sends a signed-in visitor to their account', async ({ page }) => {
    await login(page)
    await page.goto('/')

    await page.getByRole('link', { name: 'Go to your account' }).click()

    await expect(page).toHaveURL('/user')
})

test('links the hero call to action to the register page', async ({ page }) => {
    await page.goto('/')

    await page.getByRole('link', { name: 'Get started' }).click()

    await expect(page).toHaveURL('/auth/register')
})

test('renders the not-found page on an unknown route', async ({ page }) => {
    await page.goto('/does-not-exist')

    await expect(
        page.getByRole('heading', { name: 'Page not found' }),
    ).toBeVisible()

    await page.getByRole('link', { name: 'Back to home' }).click()

    await expect(page).toHaveURL('/')
    await expect(accountNav(page)).toBeVisible()
})
