import { readdir, readFile, writeFile } from 'node:fs/promises'
import path from 'node:path'
import { promisify } from 'node:util'
import { brotliCompress, constants, gzip } from 'node:zlib'

const gzipAsync = promisify(gzip)
const brotliCompressAsync = promisify(brotliCompress)

/**
 * Extensions worth pre-compressing. Images, fonts and archives are already
 * compressed and would only grow, costing build time and image size.
 */
const COMPRESSIBLE_EXTENSIONS = new Set([
    '.css',
    '.cjs',
    '.html',
    '.js',
    '.json',
    '.map',
    '.mjs',
    '.svg',
    '.txt',
    '.webmanifest',
    '.xml',
])

/**
 * Below this size the transfer is dominated by headers and round trips, and a
 * compressed variant frequently ends up larger than the original.
 */
const MINIMUM_SIZE_IN_BYTES = 1024

async function listFiles(root: string): Promise<string[]> {
    const entries = await readdir(root, { withFileTypes: true })
    const files = await Promise.all(
        entries.map((entry) => {
            const entryPath = path.join(root, entry.name)
            return entry.isDirectory()
                ? listFiles(entryPath)
                : Promise.resolve([entryPath])
        }),
    )
    return files.flat().sort()
}

/**
 * Writes `<file>.gz` and `<file>.br` next to every compressible asset of
 * `root`, so the static file server can hand out a pre-compressed payload
 * instead of compressing the same response on every request. The originals are
 * kept: clients that advertise no encoding still need them, and the server
 * resolves the original path before looking for a compressed sibling.
 */
async function compressDirectory(root: string): Promise<number> {
    const files = await listFiles(root)
    let compressedCount = 0

    for (const file of files) {
        if (!COMPRESSIBLE_EXTENSIONS.has(path.extname(file))) {
            continue
        }
        const content = await readFile(file)
        if (content.byteLength < MINIMUM_SIZE_IN_BYTES) {
            continue
        }
        const [gzipped, brotlied] = await Promise.all([
            gzipAsync(content, { level: constants.Z_BEST_COMPRESSION }),
            brotliCompressAsync(content, {
                params: {
                    [constants.BROTLI_PARAM_QUALITY]:
                        constants.BROTLI_MAX_QUALITY,
                    [constants.BROTLI_PARAM_SIZE_HINT]: content.byteLength,
                },
            }),
        ])
        await Promise.all([
            writeFile(`${file}.gz`, gzipped),
            writeFile(`${file}.br`, brotlied),
        ])
        compressedCount += 1
    }

    return compressedCount
}

const root = process.argv[2]
if (!root) {
    console.error('usage: bun scripts/compress-dist.ts <directory>')
    process.exit(1)
}

const compressedCount = await compressDirectory(path.resolve(root))
console.log(`pre-compressed ${compressedCount} files in ${root}`)
