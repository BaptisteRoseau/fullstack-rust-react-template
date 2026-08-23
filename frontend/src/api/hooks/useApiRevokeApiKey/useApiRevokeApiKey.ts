import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { apiKeyKeys, revokeApiKey } from '@/api/domains/apiKeys'

const mutationKey = (apiKeyId: string) => ['apiKeys', 'revoke', apiKeyId]

export function useApiRevokeApiKey(apiKeyId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(mutationKey(apiKeyId), () => revokeApiKey(apiKeyId), {
        onSuccess: () => void mutate(apiKeyKeys.all),
    })
}
