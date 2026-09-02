import { expect, test, type Page } from './utils/fixtures'
import { login } from './utils/session'

async function switchTo(page: Page, label: string) {
    await page.getByRole('combobox').selectOption({ label })
}

test('translates the home page into French and back', async ({ page }) => {
    await page.goto('/')

    await expect(
        page.getByRole('heading', {
            name: 'Cloud storage that secures itself',
        }),
    ).toBeVisible()

    await switchTo(page, 'Français')

    await expect(
        page.getByRole('heading', {
            name: 'Un stockage cloud qui se sécurise tout seul',
        }),
    ).toBeVisible()
    await expect(page.getByRole('link', { name: 'Commencer' })).toBeVisible()

    await switchTo(page, 'English')

    await expect(page.getByRole('link', { name: 'Get started' })).toBeVisible()
})

test('keeps the selected locale across a reload', async ({ page }) => {
    await page.goto('/')
    await switchTo(page, 'Français')
    await expect(page.getByRole('link', { name: 'Commencer' })).toBeVisible()

    await page.reload()

    await expect(page.getByRole('link', { name: 'Commencer' })).toBeVisible()
    await expect(page.getByRole('combobox')).toHaveValue('fr')
})

test('translates the account pages', async ({ page }) => {
    await login(page)
    await switchTo(page, 'Français')
    await page.goto('/user')

    await expect(
        page.getByRole('heading', { name: 'Informations' }),
    ).toBeVisible()
    await expect(page.getByLabel('Prénom')).toHaveValue('Ada')

    await page
        .getByRole('navigation', { name: 'Sections du compte' })
        .getByRole('link', { name: "Clés d'API" })
        .click()

    await expect(
        page.getByRole('heading', { name: "Clés d'API" }),
    ).toBeVisible()
    await expect(
        page.getByText("Vous n'avez pas encore de clé d'API."),
    ).toBeVisible()
})
