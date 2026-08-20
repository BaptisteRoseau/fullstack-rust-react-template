export const USERS_ENDPOINT = '/api/user'

export const userEndpoint = (userId: string) => `${USERS_ENDPOINT}/${userId}`

export type UserInfo = {
    name: string
}
