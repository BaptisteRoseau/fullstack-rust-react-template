import { readdir, readFile } from 'node:fs/promises'
import path from 'node:path'

async function listFiles(root: string, prefix = ''): Promise<string[]> {
    const entries = await readdir(path.join(root, prefix), {
        withFileTypes: true,
    })
    const files = await Promise.all(
        entries.map((entry) => {
            const relativePath = path.join(prefix, entry.name)
            return entry.isDirectory()
                ? listFiles(root, relativePath)
                : Promise.resolve([relativePath])
        }),
    )
    return files.flat().sort()
}

async function listFilesOrEmpty(root: string): Promise<string[]> {
    try {
        return await listFiles(root)
    } catch {
        return []
    }
}

/**
 * Describes what would change if `expected` were replaced by `actual`. An empty
 * result means the two trees hold byte-identical files under identical names.
 */
export async function diffDirectories(
    expected: string,
    actual: string,
): Promise<string[]> {
    const [expectedFiles, actualFiles] = await Promise.all([
        listFilesOrEmpty(expected),
        listFilesOrEmpty(actual),
    ])
    const allFiles = [...new Set([...expectedFiles, ...actualFiles])].sort()

    const differences: string[] = []
    for (const file of allFiles) {
        if (!expectedFiles.includes(file)) {
            differences.push(`missing: ${file}`)
            continue
        }
        if (!actualFiles.includes(file)) {
            differences.push(`stale: ${file}`)
            continue
        }
        const [expectedContent, actualContent] = await Promise.all([
            readFile(path.join(expected, file), 'utf8'),
            readFile(path.join(actual, file), 'utf8'),
        ])
        if (expectedContent !== actualContent) {
            differences.push(`changed: ${file}`)
        }
    }
    return differences
}
