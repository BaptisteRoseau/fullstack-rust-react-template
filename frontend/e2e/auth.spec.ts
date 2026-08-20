import { expect, test } from './utils/fixtures'
import {
    accountNav,
    expectSignedIn,
    expectSignedOut,
    login,
    logout,
    register,
} from './utils/session'

test('registers, then lands signed in', async ({ page }) => {
    await register(page)

    await expect(page).toHaveURL('/user')
    await expect(
        page.getByRole('heading', { name: 'Information' }),
    ).toBeVisible()
})

test('logs in and shows the user name in the header', async ({ page }) => {
    await login(page)

    await expect(page).toHaveURL('/')
    await expect(
        accountNav(page).getByRole('link', { name: 'Log in' }),
    ).toBeHidden()
})

test('logs out back to the home page', async ({ page }) => {
    await login(page)
    await logout(page)

    await expect(page).toHaveURL('/')
})

test('logs back in after logging out', async ({ page }) => {
    await login(page)
    await logout(page)
    await login(page)

    await expectSignedIn(page)
})

test('redirects to the login page when reaching a protected route', async ({
    page,
}) => {
    await page.goto('/user')

    await expect(page).toHaveURL(/\/auth\/login\?redirect=%2Fuser/)
    await expect(page.getByRole('heading', { name: 'Log in' })).toBeVisible()
})

test('returns to the protected route after logging in', async ({ page }) => {
    await page.goto('/user/api-keys')
    await page.getByRole('button', { name: 'Continue to sign in' }).click()

    await expect(page).toHaveURL('/user/api-keys')
    await expect(page.getByRole('heading', { name: 'API keys' })).toBeVisible()
})

test('keeps the session across a reload', async ({ page }) => {
    await login(page)
    await page.reload()

    await expectSignedIn(page)
})

test('shows the signed-out header before logging in', async ({ page }) => {
    await page.goto('/')

    await expectSignedOut(page)
    await expect(
        accountNav(page).getByRole('link', { name: 'Register' }),
    ).toBeVisible()
})

test('navigates between the login and register pages', async ({ page }) => {
    await page.goto('/auth/login')
    await page.getByRole('link', { name: 'Register' }).click()

    await expect(page).toHaveURL('/auth/register')
    await expect(
        page.getByRole('heading', { name: 'Create your account' }),
    ).toBeVisible()

    await page.getByRole('link', { name: 'Log in' }).click()

    await expect(page).toHaveURL('/auth/login')
})
