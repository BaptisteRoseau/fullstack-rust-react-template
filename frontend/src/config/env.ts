import * as z from 'zod'

const EnvSchema = z.object({
    API_URL: z.string(),
    ENABLE_API_MOCKING: z
        .string()
        .refine((value) => value === 'true' || value === 'false')
        .transform((value) => value === 'true')
        .optional(),
    APP_URL: z.string().optional().default('http://localhost:3000'),
    APP_MOCK_API_PORT: z.string().optional().default('8081'),
})

function createEnv() {
    const envVars = Object.entries(import.meta.env).reduce<
        Record<string, string>
    >((acc, [key, value]) => {
        if (key.startsWith('VITE_APP_')) {
            acc[key.replace('VITE_APP_', '')] = value
        }
        return acc
    }, {})

    const parsedEnv = EnvSchema.safeParse(envVars)

    if (!parsedEnv.success) {
        throw new Error(
            `Invalid env provided. The following variables are missing or invalid:\n${Object.entries(
                z.flattenError(parsedEnv.error).fieldErrors,
            )
                .map(([key, value]) => `- ${key}: ${value}`)
                .join('\n')}`,
        )
    }

    return parsedEnv.data
}

export const env = createEnv()
