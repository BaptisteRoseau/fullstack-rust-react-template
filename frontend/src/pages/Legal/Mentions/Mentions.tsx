import { Trans, useLingui } from '@lingui/react/macro'

import { Head } from '@/components/head/Head'

import { LegalDisclaimer } from '../components/LegalDisclaimer'
import styles from '../legal.module.scss'

export function Mentions() {
    const { t } = useLingui()

    return (
        <>
            <Head
                title={t`Legal mentions — Driftbox`}
                description={t`Placeholder legal mentions for the Driftbox proof of concept.`}
            />
            <article className={styles.document}>
                <div className={styles.inner}>
                    <h1>
                        <Trans>Legal mentions</Trans>
                    </h1>
                    <LegalDisclaimer />
                    <p className={styles.updated}>
                        <Trans>Last updated: 1 January 2026.</Trans>
                    </p>

                    <section className={styles.section}>
                        <h2>
                            <Trans>Publisher</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Driftbox is published by Driftbox SAS, a
                                fictional simplified joint-stock company with a
                                share capital of 10,000 euros, registered at 12
                                rue des Nuages, 75000 Paris, France, under the
                                imaginary trade register number 000 000 000 RCS
                                Paris.
                            </Trans>
                        </p>
                        <ul>
                            <li>
                                <Trans>
                                    Publication director: Camille Dupont,
                                    fictional managing director.
                                </Trans>
                            </li>
                            <li>
                                <Trans>
                                    Contact: legal@driftbox.example (fictional
                                    mailbox).
                                </Trans>
                            </li>
                            <li>
                                <Trans>
                                    Intra-community VAT number: FR00 000000000
                                    (fictional).
                                </Trans>
                            </li>
                        </ul>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>Hosting</Trans>
                        </h2>
                        <p>
                            <Trans>
                                In this proof of concept there is no hosting
                                company: the application runs on a container
                                stack that anyone can start on their own
                                machine. The API is a Rust service, accounts
                                live in a PostgreSQL database and uploaded files
                                live in a self-hosted distributed object store,
                                all launched side by side from a single compose
                                file.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                A real deployment would name its hosting
                                provider here, along with the registered address
                                and telephone number of that provider, as French
                                law requires.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>Intellectual property</Trans>
                        </h2>
                        <p>
                            <Trans>
                                The Driftbox name, its interface, its texts and
                                its illustrations are presented as the property
                                of Driftbox SAS. Reproducing or adapting them
                                outside the private-use exceptions provided by
                                law would require prior written permission.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                Files uploaded by users remain the property of
                                their authors. Driftbox claims no right over
                                them beyond what is strictly needed to store,
                                compress, encrypt, preview and deliver them back
                                to the people you shared them with.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>Personal data</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Driftbox processes the data strictly needed to
                                run an account: your name, your email address,
                                the team you belong to, and the metadata of the
                                files you store. The legal basis is the
                                performance of the contract formed by the terms
                                of service.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                Under the General Data Protection Regulation you
                                may access your data, correct it, ask for it to
                                be erased, object to its processing or request
                                it in a portable format. Write to
                                legal@driftbox.example; a genuine service would
                                answer within one month and would name the
                                supervisory authority you can complain to.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>Cookies and local storage</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Driftbox sets no advertising or analytics
                                cookie. Signing in stores a session cookie that
                                the browser cannot read from JavaScript, and it
                                is what keeps you logged in between visits.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                Your interface language and your light or dark
                                theme are remembered in the local storage of
                                your browser. They never leave your device and
                                clearing your browsing data removes them.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>Contact</Trans>
                        </h2>
                        <p>
                            <Trans>
                                For any question about these mentions, write to
                                legal@driftbox.example or to Driftbox SAS, 12
                                rue des Nuages, 75000 Paris, France. Both are
                                fictional and nobody reads them.
                            </Trans>
                        </p>
                    </section>
                </div>
            </article>
        </>
    )
}
