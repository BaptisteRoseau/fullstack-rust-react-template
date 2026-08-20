import { expect, test, type Page } from './utils/fixtures'
import { accountNav, login } from './utils/session'

const MOBILE = { width: 375, height: 812 }
const TABLET = { width: 768, height: 1024 }
const DESKTOP = { width: 1440, height: 900 }

async function horizontalOverflow(page: Page) {
    return page.evaluate(
        () =>
            document.documentElement.scrollWidth -
            document.documentElement.clientWidth,
    )
}

test.describe('mobile', () => {
    test.use({ viewport: MOBILE })

    test('the home page fits the viewport', async ({ page }) => {
        await page.goto('/')

        await expect(
            page.getByRole('heading', {
                name: 'Ship a fullstack app, not a toolchain',
            }),
        ).toBeVisible()
        expect(
            await horizontalOverflow(page),
            'the home page must not scroll horizontally on mobile',
        ).toBeLessThanOrEqual(0)
    })

    test('the header keeps its actions reachable', async ({ page }) => {
        await page.goto('/')

        await expect(
            accountNav(page).getByRole('link', { name: 'Log in' }),
        ).toBeVisible()
        await expect(
            accountNav(page).getByRole('link', { name: 'Register' }),
        ).toBeVisible()
        await expect(
            page.getByRole('banner').getByRole('link').first(),
        ).toBeVisible()
    })

    test('the account page stacks its navigation above the content', async ({
        page,
    }) => {
        await login(page)
        await page.goto('/user')

        const nav = page.getByRole('navigation', { name: 'Account sections' })
        const heading = page.getByRole('heading', { name: 'Information' })

        const navBox = await nav.boundingBox()
        const headingBox = await heading.boundingBox()

        expect(
            navBox,
            'the account navigation should be rendered',
        ).not.toBeNull()
        expect(
            headingBox,
            'the section heading should be rendered',
        ).not.toBeNull()
        expect(
            navBox!.y + navBox!.height,
            `expected the nav (bottom ${navBox!.y + navBox!.height}) above the heading (top ${headingBox!.y})`,
        ).toBeLessThanOrEqual(headingBox!.y)
        expect(
            await horizontalOverflow(page),
            'the account page must not scroll horizontally on mobile',
        ).toBeLessThanOrEqual(0)
    })

    test('the api keys table stays inside the viewport', async ({ page }) => {
        await login(page)
        await page.goto('/user/api-keys')

        await page.getByRole('button', { name: 'New key' }).click()
        await page.getByLabel('Name').fill('Mobile key')
        await page.getByRole('button', { name: 'Create key' }).click()

        await expect(
            page.getByRole('cell', { name: 'Mobile key', exact: true }),
        ).toBeVisible()
        expect(
            await horizontalOverflow(page),
            'the api keys page must not scroll horizontally on mobile',
        ).toBeLessThanOrEqual(0)
    })
})

test.describe('tablet', () => {
    test.use({ viewport: TABLET })

    test('the feature grid fits without overflow', async ({ page }) => {
        await page.goto('/')

        expect(
            await horizontalOverflow(page),
            'the home page must not scroll horizontally on tablet',
        ).toBeLessThanOrEqual(0)
    })
})

test.describe('desktop', () => {
    test.use({ viewport: DESKTOP })

    test('the account page puts the navigation beside the content', async ({
        page,
    }) => {
        await login(page)
        await page.goto('/user')

        const nav = page.getByRole('navigation', { name: 'Account sections' })
        const heading = page.getByRole('heading', { name: 'Information' })

        const navBox = await nav.boundingBox()
        const headingBox = await heading.boundingBox()

        expect(
            navBox!.x + navBox!.width,
            `expected the nav (right edge ${navBox!.x + navBox!.width}) left of the content (${headingBox!.x})`,
        ).toBeLessThanOrEqual(headingBox!.x)
    })

    test('the logo shows its name', async ({ page }) => {
        await page.goto('/')

        await expect(
            page.getByRole('banner').getByText('Fullstack Template'),
        ).toBeVisible()
    })
})
