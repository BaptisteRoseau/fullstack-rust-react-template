import { expect, type Page } from '@playwright/test'

export async function login(page: Page) {
    await page.goto('/auth/login')
    await page.getByRole('button', { name: 'Continue to sign in' }).click()
    await expectSignedIn(page)
}

export async function register(page: Page) {
    await page.goto('/auth/register')
    await page.getByRole('button', { name: 'Continue to registration' }).click()
    await expectSignedIn(page)
}

export async function logout(page: Page) {
    await page.getByRole('button', { name: /ada lovelace/i }).click()
    await page.getByRole('menuitem', { name: 'Log out' }).click()
    await expectSignedOut(page)
}

export function accountNav(page: Page) {
    return page.getByRole('navigation', { name: 'Account' })
}

export async function expectSignedIn(page: Page) {
    await expect(
        page.getByRole('button', { name: /ada lovelace/i }),
    ).toBeVisible()
}

export async function expectSignedOut(page: Page) {
    await expect(
        accountNav(page).getByRole('link', { name: 'Log in' }),
    ).toBeVisible()
}
