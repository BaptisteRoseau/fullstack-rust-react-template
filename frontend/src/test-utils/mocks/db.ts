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

export const db = {
    user: new Collection({ schema: userSchema }),
    apiKey: new Collection({ schema: apiKeySchema }),
}

export type Model = keyof typeof db

export type ApiKeyRecord = z.infer<typeof apiKeySchema>

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
    await persistDb('user')
    await persistDb('apiKey')
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
