import SectionContainer from '../layouts/SectionContainer'

function Header() {
    return (
        <SectionContainer>
            <nav className='flex '>
                <ul>
                    <li ><a href="/">About</a></li>
                    <li ><a href="/">Pricing</a></li>
                    <li ><a href="/">Sign In</a></li>
                    <li ><a href="/">Sign Up</a></li>
                </ul>
            </nav>
        </SectionContainer>
    )
}

export default Header
