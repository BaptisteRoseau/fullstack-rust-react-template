import { Trans } from '@lingui/macro'

import { Link } from '@/components/ui/link'
import { paths } from '@/config/paths'

const NotFoundRoute = () => {
    return (
        <div className="container mt-52 flex flex-col items-center font-semibold">
            <h1>
                <Trans>404 - Not Found</Trans>
            </h1>
            <p>
                <Trans>
                    Sorry, the page you are looking for does not exist.
                </Trans>
            </p>
            <Link to={paths.home.getHref()} replace>
                <Trans>Go to Home</Trans>
            </Link>
        </div>
    )
}

export default NotFoundRoute
