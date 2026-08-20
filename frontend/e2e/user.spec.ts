import { expect, test, type Page } from './utils/fixtures'
import { login } from './utils/session'

function sectionNav(page: Page) {
    return page.getByRole('navigation', { name: 'Account sections' })
}

async function createApiKey(page: Page, name: string, permissions?: string[]) {
    await page.getByRole('button', { name: 'New key' }).click()
    await page.getByLabel('Name').fill(name)
    for (const permission of permissions ?? []) {
        await page.getByLabel(permission, { exact: true }).check()
    }
    await page.getByRole('button', { name: 'Create key' }).click()
    await expect(
        page.getByRole('cell', { name, exact: true }),
        `the "${name}" row should be listed after creation`,
    ).toBeVisible()
}

async function revokeApiKey(page: Page, name: string) {
    await page.getByRole('button', { name: `Revoke ${name}` }).click()
    await page
        .getByRole('dialog')
        .getByRole('button', { name: 'Revoke' })
        .click()
    await expect(
        page.getByRole('cell', { name, exact: true }),
        `the "${name}" row should disappear after revocation`,
    ).toBeHidden()
}

test.beforeEach(async ({ page }) => {
    await login(page)
})

test.describe('information', () => {
    test('shows the profile facts', async ({ page }) => {
        await page.goto('/user')

        await expect(page.getByText('ada@example.com')).toBeVisible()
        await expect(page.getByText('admin')).toBeVisible()
        await expect(page.getByLabel('First name')).toHaveValue('Ada')
        await expect(page.getByLabel('Last name')).toHaveValue('Lovelace')
    })

    test('edits and persists the profile', async ({ page }) => {
        await page.goto('/user')

        await page.getByLabel('Bio').fill('Builds compilers for fun.')
        await page.getByLabel('First name').fill('Augusta')
        await page.getByRole('button', { name: 'Save changes' }).click()

        await expect(
            page.getByRole('alert', { name: 'Profile updated' }),
        ).toBeVisible()

        await page.reload()

        await expect(page.getByLabel('Bio')).toHaveValue(
            'Builds compilers for fun.',
        )
        await expect(page.getByLabel('First name')).toHaveValue('Augusta')

        await page.getByLabel('First name').fill('Ada')
        await page.getByRole('button', { name: 'Save changes' }).click()
        await expect(
            page.getByRole('alert', { name: 'Profile updated' }),
        ).toBeVisible()
    })

    test('rejects an empty required field', async ({ page }) => {
        await page.goto('/user')

        await page.getByLabel('First name').fill('')
        await page.getByRole('button', { name: 'Save changes' }).click()

        await expect(page.getByRole('alert').first()).toBeVisible()
        await expect(
            page.getByRole('alert', { name: 'Profile updated' }),
        ).toBeHidden()
    })
})

test.describe('api keys', () => {
    test('starts from an empty state', async ({ page }) => {
        await page.goto('/user/api-keys')

        await expect(
            page.getByRole('heading', { name: 'API keys' }),
        ).toBeVisible()
        await expect(page.getByText('You have no API key yet.')).toBeVisible()
    })

    test('creates a key and reveals the secret once', async ({ page }) => {
        await page.goto('/user/api-keys')

        await createApiKey(page, 'CI deploy key')

        await expect(
            page.getByText('This secret will not be shown again.'),
        ).toBeVisible()

        await page.reload()

        await expect(
            page.getByRole('cell', { name: 'CI deploy key', exact: true }),
        ).toBeVisible()
        await expect(
            page.getByText('This secret will not be shown again.'),
        ).toBeHidden()

        await revokeApiKey(page, 'CI deploy key')
    })

    test('records the selected permissions', async ({ page }) => {
        await page.goto('/user/api-keys')

        await createApiKey(page, 'Admin key', ['write', 'admin'])

        const permissions = page
            .getByRole('row', { name: /Admin key/ })
            .getByRole('cell')
            .nth(1)

        await expect(permissions).toHaveText('readwriteadmin')

        await revokeApiKey(page, 'Admin key')
    })

    test('manages several keys independently', async ({ page }) => {
        await page.goto('/user/api-keys')

        await createApiKey(page, 'First key')
        await createApiKey(page, 'Second key')
        await createApiKey(page, 'Third key')

        await expect(page.getByRole('row')).toHaveCount(4)

        await revokeApiKey(page, 'Second key')

        await expect(
            page.getByRole('cell', { name: 'First key', exact: true }),
        ).toBeVisible()
        await expect(
            page.getByRole('cell', { name: 'Third key', exact: true }),
        ).toBeVisible()
        await expect(page.getByRole('row')).toHaveCount(3)

        await revokeApiKey(page, 'First key')
        await revokeApiKey(page, 'Third key')

        await expect(page.getByText('You have no API key yet.')).toBeVisible()
    })

    test('cancels a revocation', async ({ page }) => {
        await page.goto('/user/api-keys')

        await createApiKey(page, 'Kept key')

        await page.getByRole('button', { name: 'Revoke Kept key' }).click()
        await page
            .getByRole('dialog')
            .getByRole('button', { name: 'Cancel' })
            .click()

        await expect(
            page.getByRole('cell', { name: 'Kept key', exact: true }),
        ).toBeVisible()

        await revokeApiKey(page, 'Kept key')
    })

    test('rejects a key without a name', async ({ page }) => {
        await page.goto('/user/api-keys')

        await page.getByRole('button', { name: 'New key' }).click()
        await page.getByRole('button', { name: 'Create key' }).click()

        await expect(page.getByRole('dialog')).toBeVisible()
        await expect(page.getByRole('dialog').getByRole('alert')).toBeVisible()
    })
})

test('navigates between the account sections', async ({ page }) => {
    await page.goto('/user')

    await sectionNav(page).getByRole('link', { name: 'API keys' }).click()

    await expect(page).toHaveURL('/user/api-keys')
    await expect(page.getByRole('heading', { name: 'API keys' })).toBeVisible()

    await sectionNav(page).getByRole('link', { name: 'Information' }).click()

    await expect(page).toHaveURL('/user')
    await expect(
        page.getByRole('heading', { name: 'Information' }),
    ).toBeVisible()
})

test('reaches the account page from the header menu', async ({ page }) => {
    await page.goto('/')

    await page.getByRole('button', { name: /ada lovelace/i }).click()
    await page.getByRole('menuitem', { name: 'API keys' }).click()

    await expect(page).toHaveURL('/user/api-keys')
})
