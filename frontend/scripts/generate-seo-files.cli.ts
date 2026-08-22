import path from 'node:path'
import { fileURLToPath } from 'node:url'

import { seoConfig } from '../seo.config'

import { writeSeoFiles } from './generate-seo-files'

const publicDir = fileURLToPath(new URL('../public', import.meta.url))
const writtenPaths = await writeSeoFiles(publicDir, seoConfig)

for (const writtenPath of writtenPaths) {
    console.log(`wrote ${path.relative(process.cwd(), writtenPath)}`)
}
