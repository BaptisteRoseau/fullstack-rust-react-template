type NamedPerson = {
    firstName: string
    lastName: string
    email: string
}

/** Falls back to the email address when the person has no name on record. */
export function fullName(person: NamedPerson): string {
    return `${person.firstName} ${person.lastName}`.trim() || person.email
}

export function initials(name: string): string {
    return name
        .split(/\s+/)
        .filter(Boolean)
        .slice(0, 2)
        .map((part) => part[0].toUpperCase())
        .join('')
}

export function truncate(value: string, maxLength: number): string {
    return value.length <= maxLength ? value : `${value.slice(0, maxLength)}…`
}
