---
name: frontend-react-api
description: How to declare a data-fetching or mutation endpoint (Zod schema + Axios fetcher + React Query hook) in a feature's api/ folder. Use this when creating or updating a frontend API call, query, or mutation.
---

# Adding an API Endpoint

One file per endpoint at `src/features/<feature>/api/<verb>-<noun>.ts`. Each file colocates **three
parts**: a Zod schema (for mutations/validated input), a fetcher using the shared `api` client, and a
React Query hook. Reads also export a `queryOptions` factory so loaders and components share one key.

## The single client

Always import the configured Axios instance — never raw `axios`/`fetch`:
```ts
import { api } from '@/lib/api-client'
```
Its response interceptor already unwraps `.data`, toasts errors, and redirects on 401. Note the
unwrap: when the backend returns `{ data: ... }`, your fetcher's return type reflects that envelope.

## Read (query) — `get-things.ts`

```ts
import { queryOptions, useQuery } from '@tanstack/react-query'

import { api } from '@/lib/api-client'
import { QueryConfig } from '@/lib/react-query'
import { Thing, Meta } from '@/types/api'

export const getThings = (page = 1): Promise<{ data: Thing[]; meta: Meta }> => {
    return api.get(`/things`, { params: { page } })
}

export const getThingsQueryOptions = ({ page }: { page?: number } = {}) => {
    return queryOptions({
        queryKey: page ? ['things', { page }] : ['things'],
        queryFn: () => getThings(page),
    })
}

type UseThingsOptions = {
    page?: number
    queryConfig?: QueryConfig<typeof getThingsQueryOptions>
}

export const useThings = ({ queryConfig, page }: UseThingsOptions) => {
    return useQuery({ ...getThingsQueryOptions({ page }), ...queryConfig })
}
```

Single-resource read: `getThingQueryOptions(id)` with `queryKey: ['things', id]`.

## Write (mutation) — `create-thing.ts`

Schema drives both validation and the input type; invalidate the related list on success so the
React Query cache stays fresh.

```ts
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { z } from 'zod'

import { api } from '@/lib/api-client'
import { MutationConfig } from '@/lib/react-query'
import { Thing } from '@/types/api'

import { getThingsQueryOptions } from './get-things'

export const createThingInputSchema = z.object({
    title: z.string().min(1, 'Required'),
    body: z.string().min(1, 'Required'),
})
export type CreateThingInput = z.infer<typeof createThingInputSchema>

export const createThing = ({ data }: { data: CreateThingInput }): Promise<Thing> => {
    return api.post(`/things`, data)
}

type UseCreateThingOptions = { mutationConfig?: MutationConfig<typeof createThing> }

export const useCreateThing = ({ mutationConfig }: UseCreateThingOptions = {}) => {
    const queryClient = useQueryClient()
    const { onSuccess, ...restConfig } = mutationConfig || {}
    return useMutation({
        onSuccess: (...args) => {
            queryClient.invalidateQueries({ queryKey: getThingsQueryOptions().queryKey })
            onSuccess?.(...args)
        },
        ...restConfig,
        mutationFn: createThing,
    })
}
```

## Rules

- **Always** accept a `queryConfig` / `mutationConfig` passthrough typed via `QueryConfig` / `MutationConfig` (from `@/lib/react-query`) — don't add bespoke hooks for config variants.
- **Always invalidate** the affected query key(s) in a mutation's `onSuccess`, then call the caller's `onSuccess`.
- **Query keys** are arrays scoped by resource: `['things']`, `['things', { page }]`, `['things', id]`.
- Use the `queryOptions` factory inside route `clientLoader`s (`queryClient.ensureQueryData(...)`) so pages prefetch with the same key.
- Don't catch errors here — the api-client interceptor surfaces them globally.
- Add a matching MSW handler (`frontend-react-mocks`) or the call fails in dev/tests.
