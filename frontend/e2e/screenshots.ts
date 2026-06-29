import { chromium } from 'playwright'
import { createUser } from '../src/testing/data-generators'
import * as fs from 'fs'
import * as path from 'path'

const BASE_URL = 'http://localhost:3000'
const OUT_DIR = '/tmp/screenshots'

async function run() {
    fs.mkdirSync(OUT_DIR, { recursive: true })

    const browser = await chromium.launch()
    const context = await browser.newContext({
        viewport: { width: 1280, height: 800 },
    })
    const page = await context.newPage()

    // --- Public pages ---
    await page.goto(`${BASE_URL}/`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({ path: `${OUT_DIR}/landing.png`, fullPage: true })
    console.log('✓ landing')

    await page.goto(`${BASE_URL}/auth/login`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({ path: `${OUT_DIR}/login.png`, fullPage: true })
    console.log('✓ login')

    await page.goto(`${BASE_URL}/auth/register`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({ path: `${OUT_DIR}/register.png`, fullPage: true })
    console.log('✓ register')

    await page.goto(`${BASE_URL}/this-does-not-exist`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({ path: `${OUT_DIR}/not-found.png`, fullPage: true })
    console.log('✓ not-found')

    // --- Authenticate via mock API ---
    const user = createUser()

    await page.goto(`${BASE_URL}/auth/register`)
    await page.waitForLoadState('networkidle')
    await page.getByLabel('First Name').fill(user.firstName)
    await page.getByLabel('Last Name').fill(user.lastName)
    await page.getByLabel('Email Address').fill(user.email)
    await page.getByLabel('Password').fill(user.password)
    await page.getByLabel('Team Name').fill(user.teamName)
    await page.getByRole('button', { name: 'Register' }).click()
    await page.waitForURL(`${BASE_URL}/app`)
    console.log('✓ authenticated')

    // --- Auth-protected pages ---
    await page.goto(`${BASE_URL}/app`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({ path: `${OUT_DIR}/dashboard.png`, fullPage: true })
    console.log('✓ dashboard')

    await page.goto(`${BASE_URL}/app/discussions`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({
        path: `${OUT_DIR}/discussions.png`,
        fullPage: true,
    })
    console.log('✓ discussions')

    await page.goto(`${BASE_URL}/app/profile`)
    await page.waitForLoadState('networkidle')
    await page.screenshot({ path: `${OUT_DIR}/profile.png`, fullPage: true })
    console.log('✓ profile')

    await browser.close()

    console.log(`\nScreenshots saved to ${OUT_DIR}/`)
    const files = fs.readdirSync(OUT_DIR).filter((f) => f.endsWith('.png'))
    files.forEach((f) => console.log(`  ${path.join(OUT_DIR, f)}`))
}

run().catch((err) => {
    console.error(err)
    process.exit(1)
})
