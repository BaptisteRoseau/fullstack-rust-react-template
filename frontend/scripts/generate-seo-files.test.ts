/**
 * Conformance and drift checks for the generated SEO / agent files.
 *
 * `dist/` emission (the build-time origin override performed by the `seo-files`
 * Vite plugin) is deliberately *not* covered here: a full `vite build` inside a
 * unit test is too slow. It is verified manually with
 * `SEO_SITE_URL=https://example.test bun run build`.
 *
 * The drift check assumes an unset `SEO_*` environment, matching how the
 * committed placeholder files were generated.
 */
import { existsSync, readFileSync } from 'node:fs'
import path from 'node:path'

import { PUBLIC_PATHS, seoConfig } from '../seo.config'

import { renderSeoFiles } from './generate-seo-files'

// jsdom rewrites `import.meta.url` to a non-file URL, so anchor on the Vitest
// working directory (the frontend project root) instead.
const PUBLIC_DIR = path.resolve(process.cwd(), 'public')
const SITEMAP_NAMESPACE = 'http://www.sitemaps.org/schemas/sitemap/0.9'
const EXPIRY_WARNING_DAYS = 90
const MILLISECONDS_PER_DAY = 24 * 60 * 60 * 1000

const renderedFiles = renderSeoFiles(seoConfig)

function readCommittedFile(relativePath: string): string {
    return readFileSync(path.join(PUBLIC_DIR, relativePath), 'utf8')
}

function parseXml(source: string): Document {
    const document = new DOMParser().parseFromString(
        source,
        'application/xml',
    ) as Document
    const parserError = document.querySelector('parsererror')

    expect(
        parserError,
        `expected sitemap.xml to parse as XML, got a parser error: ${parserError?.textContent}`,
    ).toBeNull()

    return document
}

function uncommentedLines(source: string): string[] {
    return source
        .split('\n')
        .map((line) => line.trim())
        .filter((line) => line.length > 0 && !line.startsWith('#'))
}

it('matches the committed files in public/ byte-for-byte', () => {
    for (const [relativePath, expectedContents] of Object.entries(
        renderedFiles,
    )) {
        const committedContents = readCommittedFile(relativePath)

        expect(
            committedContents,
            `public/${relativePath} differs from a fresh render; run "bun run seo:generate" and commit the result. Committed:\n${committedContents}\nRendered:\n${expectedContents}`,
        ).toBe(expectedContents)
    }
})

it('lists exactly the public paths in sitemap.xml', () => {
    const sitemap = parseXml(readCommittedFile('sitemap.xml'))
    const urlset = sitemap.documentElement

    expect(
        urlset.tagName,
        `expected the sitemap root element to be <urlset>, got <${urlset.tagName}>`,
    ).toBe('urlset')
    expect(
        urlset.namespaceURI,
        `expected the sitemap namespace ${SITEMAP_NAMESPACE}, got ${urlset.namespaceURI}`,
    ).toBe(SITEMAP_NAMESPACE)

    const locations = [...sitemap.querySelectorAll('url > loc')].map(
        (element) => element.textContent,
    )
    const expectedLocations = PUBLIC_PATHS.map(
        (publicPath) => `${seoConfig.siteUrl}${publicPath}`,
    )

    expect(
        locations,
        `expected the sitemap to list exactly ${JSON.stringify(expectedLocations)}, got ${JSON.stringify(locations)}`,
    ).toStrictEqual(expectedLocations)
})

it('uses absolute URLs rooted at siteUrl for every sitemap entry', () => {
    const sitemap = parseXml(readCommittedFile('sitemap.xml'))

    for (const element of sitemap.querySelectorAll('url > loc')) {
        const location = element.textContent ?? ''

        expect(
            location.startsWith(`${seoConfig.siteUrl}/`),
            `expected <loc> to be absolute and start with ${seoConfig.siteUrl}/, got ${location}`,
        ).toBe(true)
    }
})

it('declares exactly one absolute Sitemap URL in robots.txt', () => {
    const robots = readCommittedFile('robots.txt')
    const sitemapLines = uncommentedLines(robots).filter((line) =>
        line.toLowerCase().startsWith('sitemap:'),
    )

    expect(
        sitemapLines.length,
        `expected exactly one uncommented Sitemap: line in robots.txt, got ${sitemapLines.length}: ${JSON.stringify(sitemapLines)}`,
    ).toBe(1)

    const sitemapUrl = sitemapLines[0].slice('sitemap:'.length).trim()

    expect(
        URL.canParse(sitemapUrl),
        `expected the robots.txt Sitemap: value to be an absolute URL, got ${sitemapUrl}`,
    ).toBe(true)
})

it('disallows /user and keeps every AI training crawler rule commented out', () => {
    const robots = readCommittedFile('robots.txt')
    const activeLines = uncommentedLines(robots)

    expect(
        activeLines,
        `expected robots.txt to disallow /user, got ${JSON.stringify(activeLines)}`,
    ).toContain('Disallow: /user')

    for (const crawler of ['GPTBot', 'ClaudeBot', 'Google-Extended']) {
        const activeCrawlerLines = activeLines.filter((line) =>
            line.includes(crawler),
        )

        expect(
            activeCrawlerLines,
            `expected every ${crawler} rule to stay commented out (this template allows AI crawlers by default), got ${JSON.stringify(activeCrawlerLines)}`,
        ).toStrictEqual([])
    }
})

it('provides the RFC 9116 required fields in security.txt', () => {
    const securityTxt = readCommittedFile('.well-known/security.txt')
    const lines = uncommentedLines(securityTxt)

    const contactLines = lines.filter((line) => line.startsWith('Contact:'))
    const expiresLines = lines.filter((line) => line.startsWith('Expires:'))

    expect(
        contactLines.length,
        `expected at least one Contact: line in security.txt, got ${contactLines.length}`,
    ).toBeGreaterThanOrEqual(1)
    expect(
        expiresLines.length,
        `expected exactly one Expires: line in security.txt, got ${expiresLines.length}: ${JSON.stringify(expiresLines)}`,
    ).toBe(1)

    const expiresValue = expiresLines[0].slice('Expires:'.length).trim()
    const expiresAt = new Date(expiresValue)

    expect(
        Number.isNaN(expiresAt.getTime()),
        `expected the Expires: value to be a parseable ISO-8601 timestamp, got ${expiresValue}`,
    ).toBe(false)

    const remainingDays = Math.floor(
        (expiresAt.getTime() - Date.now()) / MILLISECONDS_PER_DAY,
    )

    expect(
        remainingDays,
        `security.txt expired ${Math.abs(remainingDays)} days ago (Expires: ${expiresValue}); bump securityExpires in seo.config.ts and regenerate`,
    ).toBeGreaterThan(0)

    if (remainingDays < EXPIRY_WARNING_DAYS) {
        console.warn(
            `security.txt expires in ${remainingDays} days (Expires: ${expiresValue}); bump securityExpires in seo.config.ts soon`,
        )
    }
})

it('follows the llms.txt v2 structure', () => {
    const llmsTxt = readCommittedFile('llms.txt')
    const lines = llmsTxt.split('\n')

    const h1Lines = lines.filter((line) => /^# /.test(line))

    expect(
        h1Lines.length,
        `expected exactly one H1 in llms.txt, got ${h1Lines.length}: ${JSON.stringify(h1Lines)}`,
    ).toBe(1)
    expect(
        lines[0],
        `expected llms.txt to start with its H1, got ${JSON.stringify(lines[0])}`,
    ).toBe(h1Lines[0])

    const deeperHeadings = lines.filter((line) => /^#{3,} /.test(line))

    expect(
        deeperHeadings,
        `expected llms.txt to use only H1 and H2 headings, got ${JSON.stringify(deeperHeadings)}`,
    ).toStrictEqual([])

    let currentSection: string | null = null

    for (const line of lines) {
        if (/^## /.test(line)) {
            currentSection = line
            continue
        }

        if (currentSection === null || line.trim().length === 0) {
            continue
        }

        expect(
            /^- \[[^\]]+\]\([^)]+\)(: .+)?$/.test(line),
            `expected every line in the "${currentSection}" section of llms.txt to be a "- [name](url)" link item, got ${JSON.stringify(line)}`,
        ).toBe(true)
    }
})

it('points the web manifest at icons that exist', () => {
    const manifestSource = readCommittedFile('site.webmanifest')
    const manifest = JSON.parse(manifestSource) as {
        icons: { src: string }[]
    }

    expect(
        manifest.icons.length,
        `expected the manifest to declare at least one icon, got ${manifest.icons.length}`,
    ).toBeGreaterThan(0)

    for (const icon of manifest.icons) {
        const iconPath = path.join(PUBLIC_DIR, icon.src)

        expect(
            existsSync(iconPath),
            `expected the manifest icon ${icon.src} to exist at ${iconPath}`,
        ).toBe(true)
    }
})
