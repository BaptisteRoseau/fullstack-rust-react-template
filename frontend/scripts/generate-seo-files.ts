import { mkdir, writeFile } from 'node:fs/promises'
import path from 'node:path'

import { PUBLIC_PATHS, type SeoConfig } from '../seo.config'
import { PATHS } from '../src/router/constants'

type PublicPath = (typeof PUBLIC_PATHS)[number]

/**
 * Human labels for the `llms.txt` link list. Typed against `PUBLIC_PATHS` so
 * publishing a new path without describing it is a compile error.
 */
const PAGE_LABELS: Record<PublicPath, { name: string; note: string }> = {
    [PATHS.home]: { name: 'Home', note: 'Application landing page' },
    [PATHS.login]: { name: 'Sign in', note: 'Session login' },
    [PATHS.register]: { name: 'Register', note: 'Account creation' },
    [PATHS.legal.terms]: {
        name: 'Terms of Service',
        note: 'Placeholder terms of service for this proof of concept',
    },
    [PATHS.legal.mentions]: {
        name: 'Legal mentions',
        note: 'Placeholder publisher, hosting and personal-data notice',
    },
}

const XML_ENTITIES: Record<string, string> = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&apos;',
}

function escapeXml(value: string): string {
    return value.replace(/[&<>"']/g, (character) => XML_ENTITIES[character])
}

function toAbsoluteUrl(siteUrl: string, publicPath: string): string {
    return `${siteUrl}${publicPath}`
}

function renderRobotsTxt(config: SeoConfig): string {
    return `# https://www.robotstxt.org/robotstxt.html
User-agent: *
Allow: /
Disallow: /user

Sitemap: ${config.siteUrl}/sitemap.xml

# Disallow above controls crawling, not indexing: a disallowed URL can still be
# indexed without a snippet. Use an X-Robots-Tag response header to prevent
# indexing.
#
# This template allows all AI crawlers by default. Uncomment the block below to
# opt out of AI *training* crawls while remaining eligible for AI-search
# citations (OAI-SearchBot, Claude-SearchBot and PerplexityBot stay allowed).
# Google-Extended is training-only and does not affect Google Search ranking.
# Compliance is voluntary: these are requests, not enforcement.
#
# User-agent: GPTBot
# User-agent: ClaudeBot
# User-agent: CCBot
# User-agent: Google-Extended
# User-agent: Applebot-Extended
# User-agent: Bytespider
# User-agent: FacebookBot
# Disallow: /
`
}

function renderSitemapXml(config: SeoConfig): string {
    const urls = PUBLIC_PATHS.map(
        (publicPath) => `    <url>
        <loc>${escapeXml(toAbsoluteUrl(config.siteUrl, publicPath))}</loc>
    </url>`,
    ).join('\n')

    return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>
`
}

function renderSecurityTxt(config: SeoConfig): string {
    const lines = [
        `Contact: ${config.securityContact}`,
        `Expires: ${config.securityExpires}`,
    ]

    if (config.securityPolicyUrl) {
        lines.push(`Policy: ${config.securityPolicyUrl}`)
    }

    lines.push(
        'Preferred-Languages: en, fr',
        `Canonical: ${config.siteUrl}/.well-known/security.txt`,
    )

    return `${lines.join('\n')}\n`
}

function renderLlmsTxt(config: SeoConfig): string {
    const pages = PUBLIC_PATHS.map((publicPath) => {
        const { name, note } = PAGE_LABELS[publicPath]
        return `- [${name}](${toAbsoluteUrl(config.siteUrl, publicPath)}): ${note}`
    }).join('\n')

    return `# ${config.siteName}

> ${config.siteDescription}

This is a client-rendered React single-page application with a Rust backend. Only the
pages listed below are reachable without authentication; everything under /user requires
a signed-in session.

## Pages

${pages}
`
}

function renderWebManifest(config: SeoConfig): string {
    return `{
    "id": "${config.siteUrl}/",
    "name": "${config.siteName}",
    "short_name": "${config.siteName}",
    "description": "${config.siteDescription}",
    "start_url": "/",
    "scope": "/",
    "display": "standalone",
    "background_color": "#ffffff",
    "theme_color": "${config.themeColor}",
    "icons": [
        { "src": "/logo192.png", "sizes": "192x192", "type": "image/png" },
        { "src": "/logo512.png", "sizes": "512x512", "type": "image/png" },
        {
            "src": "/logo512.png",
            "sizes": "512x512",
            "type": "image/png",
            "purpose": "maskable"
        }
    ]
}
`
}

/**
 * Renders every SEO and agent-discoverability file. Pure by contract: no clock,
 * no filesystem, no environment reads, so the drift test can compare a fresh
 * render against the committed files byte-for-byte.
 *
 * Keys are paths relative to `public/`.
 */
export function renderSeoFiles(config: SeoConfig): Record<string, string> {
    return {
        'robots.txt': renderRobotsTxt(config),
        'sitemap.xml': renderSitemapXml(config),
        'site.webmanifest': renderWebManifest(config),
        'llms.txt': renderLlmsTxt(config),
        '.well-known/security.txt': renderSecurityTxt(config),
    }
}

export async function writeSeoFiles(
    outputDir: string,
    config: SeoConfig,
): Promise<string[]> {
    const files = renderSeoFiles(config)
    const writtenPaths: string[] = []

    for (const [relativePath, contents] of Object.entries(files)) {
        const filePath = path.join(outputDir, relativePath)
        await mkdir(path.dirname(filePath), { recursive: true })
        await writeFile(filePath, contents, 'utf8')
        writtenPaths.push(filePath)
    }

    return writtenPaths
}
