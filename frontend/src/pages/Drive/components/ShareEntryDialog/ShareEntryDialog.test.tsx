import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

import { useApiFilePermissions } from '@/api/hooks/useApiFilePermissions'
import { useApiGrantFilePermission } from '@/api/hooks/useApiGrantFilePermission'
import { useApiRevokeFilePermission } from '@/api/hooks/useApiRevokeFilePermission'
import { buildPermissionGrant } from '@/test-utils/fixtures/drive'
import { render } from '@/test-utils/render'

import { ShareEntryDialog } from './ShareEntryDialog'

vi.mock('@/api/hooks/useApiFilePermissions')
vi.mock('@/api/hooks/useApiDirectoryPermissions')
vi.mock('@/api/hooks/useApiGrantFilePermission')
vi.mock('@/api/hooks/useApiGrantDirectoryPermission')
vi.mock('@/api/hooks/useApiRevokeFilePermission')
vi.mock('@/api/hooks/useApiRevokeDirectoryPermission')

const grantTrigger = vi.fn()
const revokeTrigger = vi.fn()

function mockGrants(result: {
    data?: ReturnType<typeof buildPermissionGrant>[]
    error?: unknown
    isLoading?: boolean
}) {
    vi.mocked(useApiFilePermissions).mockReturnValue({
        data: result.data,
        error: result.error,
        isLoading: result.isLoading ?? false,
        isValidating: false,
        mutate: vi.fn(),
    } as unknown as ReturnType<typeof useApiFilePermissions>)
}

beforeEach(() => {
    grantTrigger.mockResolvedValue(buildPermissionGrant())
    revokeTrigger.mockResolvedValue(undefined)
    vi.mocked(useApiGrantFilePermission).mockReturnValue({
        trigger: grantTrigger,
        isMutating: false,
    } as unknown as ReturnType<typeof useApiGrantFilePermission>)
    vi.mocked(useApiRevokeFilePermission).mockReturnValue({
        trigger: revokeTrigger,
        isMutating: false,
    } as unknown as ReturnType<typeof useApiRevokeFilePermission>)
    mockGrants({ data: [] })
})

function renderDialog() {
    return render(
        <ShareEntryDialog
            kind="file"
            entryId="file-42"
            name="notes.txt"
            isOpen
            onOpenChange={vi.fn()}
        />,
    )
}

it('says so when nobody else has access', () => {
    renderDialog()

    expect(
        screen.getByText('Nobody else has access yet.'),
        `expected the empty state, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('reports grants that could not be loaded', () => {
    mockGrants({ data: undefined, error: new Error('boom') })

    renderDialog()

    expect(
        screen.getByRole('alert'),
        `expected the error state, got: ${document.body.textContent}`,
    ).toHaveTextContent('The people with access could not be loaded.')
})

it('grants the level the user picked to the id they pasted', async () => {
    renderDialog()

    await userEvent.type(
        screen.getByLabelText('User ID'),
        '11111111-1111-4111-8111-111111111111',
    )
    await userEvent.selectOptions(
        screen.getByLabelText('Access level'),
        'editor',
    )
    await userEvent.click(screen.getByRole('button', { name: 'Share' }))

    expect(
        grantTrigger.mock.calls[0]?.[0],
        `expected the pasted id as an editor, got ${JSON.stringify(grantTrigger.mock.calls[0]?.[0])}`,
    ).toEqual({
        userId: '11111111-1111-4111-8111-111111111111',
        level: 'editor',
    })
})

it('refuses anything that is not a user id', async () => {
    renderDialog()

    await userEvent.type(screen.getByLabelText('User ID'), 'ada@example.com')
    await userEvent.click(screen.getByRole('button', { name: 'Share' }))

    expect(
        grantTrigger,
        'an address is not a user id and must not reach the backend',
    ).not.toHaveBeenCalled()
})

it('revokes the grant the row stands for', async () => {
    mockGrants({ data: [buildPermissionGrant({ grantee: 'user-7' })] })

    renderDialog()

    await userEvent.click(
        screen.getByRole('button', { name: 'Revoke the access of user-7' }),
    )

    expect(
        revokeTrigger.mock.calls[0]?.[0],
        `expected user-7 to be revoked, got ${JSON.stringify(revokeTrigger.mock.calls[0]?.[0])}`,
    ).toBe('user-7')
})
