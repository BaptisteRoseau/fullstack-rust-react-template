import * as z from 'zod'

import { PATHS } from './src/router/constants'

const SeoConfigSchema = z.object({
    siteUrl: z
        .url('must be an absolute URL')
        .transform((value) => value.replace(/\/+$/, '')),
    siteName: z.string().min(1, 'must not be empty'),
    siteDescription: z.string().min(1, 'must not be empty'),
    themeColor: z
        .string()
        .regex(/^#[0-9a-f]{3,8}$/i, 'must be a hexadecimal CSS color'),
    securityContact: z
        .string()
        .regex(
            /^(mailto:|tel:|https:\/\/)/,
            'must start with mailto:, tel: or https:// (RFC 9116)',
        ),
    securityPolicyUrl: z.url('must be an absolute URL').optional(),
    securityExpires: z.iso.datetime('must be an ISO-8601 timestamp'),
})

export type SeoConfig = z.infer<typeof SeoConfigSchema>

/**
 * Paths published to crawlers and agents. This list opts *in*: a new entry in
 * `PATHS` stays unpublished until it is added here, so authenticated or
 * half-finished routes never leak into the sitemap.
 */
export const PUBLIC_PATHS = [PATHS.home, PATHS.login, PATHS.register] as const

function createSeoConfig(): SeoConfig {
    const rawConfig = {
        siteUrl:
            process.env.SEO_SITE_URL ??
            process.env.VITE_APP_APP_URL ??
            'http://localhost:3000',
        siteName: process.env.SEO_SITE_NAME ?? 'Fullstack Template',
        siteDescription:
            process.env.SEO_SITE_DESCRIPTION ??
            'A Rust and React fullstack application template.',
        themeColor: process.env.SEO_THEME_COLOR ?? '#2f6feb',
        // Placeholder address: replace it per deployment with a mailbox that a
        // human actually reads, or security reports will go nowhere.
        securityContact:
            process.env.SEO_SECURITY_CONTACT ?? 'mailto:security@example.com',
        securityPolicyUrl: process.env.SEO_SECURITY_POLICY_URL,
        securityExpires:
            process.env.SEO_SECURITY_EXPIRES ?? '2027-01-01T00:00:00.000Z',
    }

    const parsedConfig = SeoConfigSchema.safeParse(rawConfig)

    if (!parsedConfig.success) {
        throw new Error(
            `Invalid SEO configuration. The following fields are missing or invalid:\n${Object.entries(
                z.flattenError(parsedConfig.error).fieldErrors,
            )
                .map(
                    ([key, errors]) =>
                        `- ${key}: ${errors?.join(', ')} (received: ${JSON.stringify(
                            rawConfig[key as keyof typeof rawConfig],
                        )})`,
                )
                .join('\n')}`,
        )
    }

    return parsedConfig.data
}

export const seoConfig = createSeoConfig()
