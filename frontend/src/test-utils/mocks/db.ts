import { Collection } from '@msw/data'
import { nanoid } from 'nanoid'
import { z } from 'zod'

/**
 * Collections are described with Zod rather than a bespoke model DSL: `@msw/data`
 * speaks Standard Schema, so the record types the handlers see are the schema's
 * own output types. That is what lets `permissions` be a real `string[]` and
 * `createdAt` carry the wire's type per collection — an RFC 3339 string for an
 * API key, an epoch in milliseconds for a user.
 */
const userSchema = z.object({
    id: z.string().default(() => nanoid()),
    firstName: z.string(),
    lastName: z.string(),
    email: z.string(),
    role: z.string(),
    teamId: z.string(),
    createdAt: z.number(),
})

const apiKeySchema = z.object({
    id: z.string().default(() => nanoid()),
    name: z.string(),
    permissions: z.array(z.string()),
    createdAt: z.string(),
    userId: z.string(),
})

const entrySchema = {
    id: z.string().default(() => nanoid()),
    name: z.string(),
    owner: z.string(),
    parentId: z.string().nullable().default(null),
    createdAt: z.string(),
    updatedAt: z.string(),
}

const directorySchema = z.object(entrySchema)

/**
 * `content` holds the bytes as base64 so the record survives the JSON file the
 * dev and e2e databases are persisted to; `storedSizeBytes` mimics the
 * backend's compression by reporting less than what was handed over.
 */
const fileSchema = z.object({
    ...entrySchema,
    mimeType: z.string(),
    sizeBytes: z.number(),
    storedSizeBytes: z.number(),
    hasThumbnail: z.boolean(),
    content: z.string(),
})

const permissionSchema = {
    id: z.string().default(() => nanoid()),
    grantee: z.string(),
    grantedBy: z.string(),
    level: z.enum(['viewer', 'editor', 'manager']),
    createdAt: z.string(),
    updatedAt: z.string(),
}

const directoryPermissionSchema = z.object({
    ...permissionSchema,
    directoryId: z.string(),
})

const filePermissionSchema = z.object({
    ...permissionSchema,
    fileId: z.string(),
})

export const db = {
    user: new Collection({ schema: userSchema }),
    apiKey: new Collection({ schema: apiKeySchema }),
    directory: new Collection({ schema: directorySchema }),
    file: new Collection({ schema: fileSchema }),
    directoryPermission: new Collection({ schema: directoryPermissionSchema }),
    filePermission: new Collection({ schema: filePermissionSchema }),
}

export type Model = keyof typeof db

export type ApiKeyRecord = z.infer<typeof apiKeySchema>

export type DirectoryRecord = z.infer<typeof directorySchema>

export type FileRecord = z.infer<typeof fileSchema>

export type DirectoryPermissionRecord = z.infer<
    typeof directoryPermissionSchema
>

export type FilePermissionRecord = z.infer<typeof filePermissionSchema>

const DB_FILE_PATH = 'mocked-db.json'
const DB_STORAGE_KEY = 'msw-db'

export const CURRENT_USER_ID = 'current-user'

async function loadDb(): Promise<Record<string, unknown[]>> {
    if (typeof window === 'undefined') {
        const { readFile, writeFile } = await import('fs/promises')
        try {
            return JSON.parse(await readFile(DB_FILE_PATH, 'utf8'))
        } catch {
            await writeFile(DB_FILE_PATH, '{}')
            return {}
        }
    }
    return JSON.parse(window.localStorage.getItem(DB_STORAGE_KEY) ?? '{}')
}

async function storeDb(data: string) {
    if (typeof window === 'undefined') {
        const { writeFile } = await import('fs/promises')
        await writeFile(DB_FILE_PATH, data)
        return
    }
    window.localStorage.setItem(DB_STORAGE_KEY, data)
}

export async function persistDb(model: Model) {
    if (process.env.NODE_ENV === 'test') {
        return
    }
    const data = await loadDb()
    data[model] = db[model].all()
    await storeDb(JSON.stringify(data))
}

export async function seedDb() {
    if (db.user.findFirst((query) => query.where({ id: CURRENT_USER_ID }))) {
        return
    }
    await db.user.create({
        id: CURRENT_USER_ID,
        firstName: 'Ada',
        lastName: 'Lovelace',
        email: 'ada@example.com',
        role: 'admin',
        teamId: 'team-1',
        createdAt: Date.UTC(2026, 0, 15),
    })
    const now = new Date(Date.UTC(2026, 0, 15)).toISOString()
    const invoices = await db.directory.create({
        id: 'seed-directory',
        name: 'Invoices',
        owner: CURRENT_USER_ID,
        parentId: null,
        createdAt: now,
        updatedAt: now,
    })
    const welcome = 'Welcome to Driftbox.\n'
    await db.file.create({
        id: 'seed-file',
        name: 'welcome.txt',
        owner: CURRENT_USER_ID,
        parentId: invoices.parentId,
        mimeType: 'text/plain',
        sizeBytes: welcome.length,
        storedSizeBytes: Math.ceil(welcome.length * 0.6),
        hasThumbnail: false,
        content: btoa(welcome),
        createdAt: now,
        updatedAt: now,
    })
    await persistDb('user')
    await persistDb('apiKey')
    await persistDb('directory')
    await persistDb('file')
}

export async function initializeDb() {
    const database = await loadDb()
    for (const [key, collection] of Object.entries(db)) {
        for (const entry of database[key] ?? []) {
            await collection.create(entry as never)
        }
    }
    await seedDb()
}

export function resetDb() {
    if (typeof window !== 'undefined') {
        window.localStorage.clear()
    }
    Object.values(db).forEach((collection) => {
        collection.clear()
    })
}
