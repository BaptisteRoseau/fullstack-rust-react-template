import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import * as z from 'zod'

import { Form } from '@/components/forms/Form'
import { render } from '@/test-utils/render'

import { SelectField } from './SelectField'

const schema = z.object({ level: z.enum(['viewer', 'editor']) })

function TestForm({ onSubmit }: { onSubmit: (values: unknown) => void }) {
    return (
        <Form
            schema={schema}
            onSubmit={onSubmit}
            defaultValues={{ level: 'viewer' }}
        >
            {() => (
                <>
                    <SelectField
                        name="level"
                        label="Level"
                        options={[
                            { value: 'viewer', label: 'Viewer' },
                            { value: 'editor', label: 'Editor' },
                        ]}
                    />
                    <button type="submit">Save</button>
                </>
            )}
        </Form>
    )
}

it('submits the option the user picked', async () => {
    const onSubmit = vi.fn()
    render(<TestForm onSubmit={onSubmit} />)

    await userEvent.selectOptions(screen.getByLabelText('Level'), 'editor')
    await userEvent.click(screen.getByRole('button', { name: 'Save' }))

    expect(
        onSubmit.mock.calls[0]?.[0],
        `expected the editor level, got ${JSON.stringify(onSubmit.mock.calls[0]?.[0])}`,
    ).toEqual({ level: 'editor' })
})
