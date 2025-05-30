import * as React from 'react'

interface SectionContainerLayoutProps {
    children: React.ReactNode
}

function SectionContainer({ children }: SectionContainerLayoutProps) {
    return (
        <>
            <section className="container">{children}</section>
        </>
    )
}

export default SectionContainer
