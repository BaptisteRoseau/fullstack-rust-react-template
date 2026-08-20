export type Nullable<T> = T | null

export type PaginatedResponse<T> = {
    results: T[]
    totalCount: number
}
