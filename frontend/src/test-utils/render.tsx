import { render as rtlRender, type RenderOptions } from '@testing-library/react'
import { MemoryRouter } from 'react-router'
import { SWRConfig } from 'swr'

import { Context } from '@/Context'

export function render(
    ui: React.ReactElement,
    { route = '/', ...options }: RenderOptions & { route?: string } = {},
) {
    function Wrapper({ children }: { children: React.ReactNode }) {
        return (
            <SWRConfig
                value={{ provider: () => new Map(), dedupingInterval: 0 }}
            >
                <Context>
                    <MemoryRouter initialEntries={[route]}>
                        {children}
                    </MemoryRouter>
                </Context>
            </SWRConfig>
        )
    }

    return rtlRender(ui, { wrapper: Wrapper, ...options })
}
