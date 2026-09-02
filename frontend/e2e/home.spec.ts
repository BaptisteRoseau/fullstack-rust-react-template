import { expect, test } from './utils/fixtures'
import { accountNav, expectSignedOut, login } from './utils/session'

test('shows the hero, the features and the footer', async ({ page }) => {
    await page.goto('/')

    await expect(
        page.getByRole('heading', {
            name: 'Cloud storage that secures itself',
        }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'Compressed and encrypted' }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'Instant previews' }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'Sharing you can aim' }),
    ).toBeVisible()
    await expect(
        page.getByRole('heading', { name: 'Built for several people' }),
    ).toBeVisible()
    await expect(page.getByRole('contentinfo')).toBeVisible()
    await expectSignedOut(page)
})

test('sends a signed-in visitor to their drive', async ({ page }) => {
    await login(page)
    await page.goto('/')

    await expect(
        page.getByRole('link', { name: 'Open your drive' }),
    ).toHaveAttribute('href', '/drive')
})

test('links the footer to the legal pages', async ({ page }) => {
    await page.goto('/')

    await page
        .getByRole('navigation', { name: 'Footer' })
        .getByRole('link', { name: 'Terms of Service' })
        .click()

    await expect(page).toHaveURL('/legal/terms')
    await expect(
        page.getByRole('heading', { name: 'Terms of Service', level: 1 }),
    ).toBeVisible()

    await page
        .getByRole('navigation', { name: 'Footer' })
        .getByRole('link', { name: 'Legal mentions' })
        .click()

    await expect(page).toHaveURL('/legal/mentions')
    await expect(
        page.getByRole('heading', { name: 'Legal mentions', level: 1 }),
    ).toBeVisible()
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
