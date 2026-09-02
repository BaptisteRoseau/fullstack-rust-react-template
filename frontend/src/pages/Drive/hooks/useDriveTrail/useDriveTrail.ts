import { useLocation } from 'react-router'

import type { DriveDirectory } from '@/api/domains/drive'

import type { DriveTrailEntry } from '../../types'

export type DriveTrail = {
    entries: DriveTrailEntry[]
    /** `false` when the ancestors above the current directory are unknown. */
    isComplete: boolean
}

type DriveLocationState = { trail?: DriveTrailEntry[] } | null

/**
 * The backend has no ancestor-chain endpoint: a listing names the directory it
 * lists and its parent's id, never its grandparents' names. So the trail is
 * accumulated as the user walks in, carried in the router's location state —
 * which makes back and forward restore it for free.
 *
 * A deep link opened cold therefore knows only where it landed, and says so by
 * eliding the steps above it. The exception is a folder sitting at the root:
 * its own `parentId` proves there is nothing between it and Home.
 */
export function useDriveTrail(
    directoryId: string | undefined,
    directory: DriveDirectory | null | undefined,
): DriveTrail {
    const { state } = useLocation()
    const carried = (state as DriveLocationState)?.trail

    if (!directoryId) {
        return { entries: [], isComplete: true }
    }

    if (carried?.at(-1)?.id === directoryId) {
        return { entries: carried, isComplete: true }
    }

    return {
        entries: [{ id: directoryId, name: directory?.name ?? '' }],
        isComplete: directory?.parentId === null,
    }
}
