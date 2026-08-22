import { existsSync } from 'node:fs'
import { mkdtemp, rm } from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

import { createClient } from '@hey-api/openapi-ts'

import { diffDirectories } from './diffDirectories'

const packageRoot = fileURLToPath(new URL('..', import.meta.url))
const SDK_PATH = path.join(packageRoot, 'src/api/generated')
const DEFAULT_SPEC_PATH = path.join(packageRoot, 'openapi.json')
const SCRATCH_PREFIX = path.join(packageRoot, '.api-sdk-check-')

/**
 * Generates the SDK from an OpenAPI document into `outputPath`.
 *
 * `auth: false` matters: the document advertises API-key and OIDC security
 * schemes, but the browser authenticates with httpOnly cookies through the
 * backend-for-frontend, so an `Authorization` header must never be attached.
 * `enums: false` keeps `ApiErrorId` a plain union, which `src/api/errors.ts`
 * narrows on. `clean` drops orphans so a deleted endpoint leaves no file
 * behind in a committed folder.
 */
async function generate(specPath: string, outputPath: string): Promise<void> {
    await createClient({
        input: specPath,
        logs: { level: 'warn' },
        output: {
            clean: true,
            path: outputPath,
            postProcess: ['prettier'],
        },
        plugins: [
            { name: '@hey-api/client-fetch' },
            { enums: false, name: '@hey-api/typescript' },
            { auth: false, name: '@hey-api/sdk' },
        ],
    })
}

/**
 * Generates into a scratch directory *inside the package*, and outside
 * `node_modules`. The `prettier` post-processor resolves `.prettierrc` by
 * walking up from the files it formats, so an output folder elsewhere comes
 * back with Prettier's defaults and differs on every line; and Prettier refuses
 * to format anything under `node_modules` at all.
 */
async function generateIntoScratchDirectory(specPath: string): Promise<string> {
    const scratchPath = await mkdtemp(SCRATCH_PREFIX)
    await generate(specPath, scratchPath)
    return scratchPath
}

async function check(specPath: string): Promise<void> {
    const scratchPath = await generateIntoScratchDirectory(specPath)
    try {
        const differences = await diffDirectories(SDK_PATH, scratchPath)
        if (differences.length === 0) {
            return
        }
        console.error(
            `src/api/generated is out of date with ${path.relative(process.cwd(), specPath)}:`,
        )
        for (const difference of differences) {
            console.error(`  ${difference}`)
        }
        console.error('Run ./scripts/build_frontend_api_sdk.sh to regenerate.')
        process.exit(1)
    } finally {
        await rm(scratchPath, { force: true, recursive: true })
    }
}

const positionals = process.argv.slice(2).filter((arg) => arg !== '--check')
const specPath = path.resolve(positionals[0] ?? DEFAULT_SPEC_PATH)

if (!existsSync(specPath)) {
    console.error(
        `OpenAPI spec not found at ${specPath}. Run ./scripts/build_frontend_api_sdk.sh to produce it.`,
    )
    process.exit(1)
}

if (process.argv.includes('--check')) {
    await check(specPath)
} else {
    await generate(specPath, SDK_PATH)
    console.log(`wrote ${path.relative(process.cwd(), SDK_PATH)}`)
}
