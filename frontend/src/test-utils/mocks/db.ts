import { factory, primaryKey } from '@mswjs/data'
import { nanoid } from 'nanoid'

const models = {
    user: {
        id: primaryKey(nanoid),
        firstName: String,
        lastName: String,
        email: String,
        role: String,
        teamId: String,
        createdAt: Number,
    },
    apiKey: {
        id: primaryKey(nanoid),
        name: String,
        permissions: Array,
        createdAt: String,
        userId: String,
    },
}

export const db = factory(models)

export type Model = keyof typeof models

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
    data[model] = db[model].getAll()
    await storeDb(JSON.stringify(data))
}

export async function seedDb() {
    if (db.user.findFirst({ where: { id: { equals: CURRENT_USER_ID } } })) {
        return
    }
    db.user.create({
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
    Object.entries(db).forEach(([key, model]) => {
        database[key]?.forEach((entry) => {
            model.create(entry as never)
        })
    })
    await seedDb()
}

export function resetDb() {
    if (typeof window !== 'undefined') {
        window.localStorage.clear()
    }
    Object.values(db).forEach((model) => {
        model.deleteMany({ where: {} })
    })
}
