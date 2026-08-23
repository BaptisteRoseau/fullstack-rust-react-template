import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import {
    currentUserKeys,
    updateCurrentUser,
    type ProfileUpdate,
} from '@/api/domains/currentUser'

const MUTATION_KEY = ['currentUser', 'update'] as const

export function useApiUpdateCurrentUser() {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        MUTATION_KEY,
        (_key, { arg }: { arg: ProfileUpdate }) => updateCurrentUser(arg),
        { onSuccess: () => void mutate(currentUserKeys.me) },
    )
}
