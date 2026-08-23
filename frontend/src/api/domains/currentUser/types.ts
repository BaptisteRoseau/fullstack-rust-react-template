export type CurrentUser = {
    id: string
    email: string
    firstName: string
    lastName: string
    /** Free-form: the backend has no closed set of roles yet. */
    role: string
    teamId: string
    createdAt: Date
}

export type ProfileUpdate = Pick<CurrentUser, 'firstName' | 'lastName'>
