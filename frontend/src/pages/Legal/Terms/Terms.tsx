import { Trans, useLingui } from '@lingui/react/macro'

import { Head } from '@/components/head/Head'

import { LegalDisclaimer } from '../components/LegalDisclaimer'
import styles from '../legal.module.scss'

export function Terms() {
    const { t } = useLingui()

    return (
        <>
            <Head
                title={t`Terms of Service — Driftbox`}
                description={t`Placeholder terms of service for the Driftbox proof of concept.`}
            />
            <article className={styles.document}>
                <div className={styles.inner}>
                    <h1>
                        <Trans>Terms of Service</Trans>
                    </h1>
                    <LegalDisclaimer />
                    <p className={styles.updated}>
                        <Trans>Last updated: 1 January 2026.</Trans>
                    </p>

                    <section className={styles.section}>
                        <h2>
                            <Trans>1. Acceptance of these terms</Trans>
                        </h2>
                        <p>
                            <Trans>
                                These terms form the agreement between Driftbox
                                SAS, a fictional company registered at 12 rue
                                des Nuages, 75000 Paris, France, and any person
                                who creates an account on Driftbox or uses the
                                service in any way. By registering an account,
                                uploading a file or opening a link that someone
                                shared with you, you accept these terms in full.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                If you do not agree with any part of them, do
                                not create an account and do not use the
                                service. If you accept them on behalf of an
                                organisation, you confirm that you are allowed
                                to bind that organisation.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>2. What Driftbox does</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Driftbox is a multi-user cloud storage service.
                                It lets you upload files, organise them into
                                folders, browse previews of images and
                                documents, and share individual files or folders
                                with other members of your team.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                Files are compressed and encrypted
                                automatically, without any action on your part.
                                Sharing is granular: each share names a single
                                file or a single folder and gives the recipient
                                one of three roles — viewer, editor or manager.
                                A share never grants access to anything above
                                the item you picked.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>3. Your account</Trans>
                        </h2>
                        <p>
                            <Trans>
                                You are responsible for the accuracy of the
                                information on your account, for keeping your
                                credentials confidential, and for everything
                                that happens under your account. Tell us
                                immediately if you believe someone else has
                                gained access to it.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                Accounts are personal. Do not share a single
                                account between several people: invite them
                                instead, so that each person signs in with their
                                own identity and each action can be traced back
                                to a real user.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>4. Acceptable use</Trans>
                        </h2>
                        <p>
                            <Trans>
                                You keep every right you already had over the
                                files you upload. In exchange, you agree not to
                                use Driftbox to:
                            </Trans>
                        </p>
                        <ul>
                            <li>
                                <Trans>
                                    store or distribute content that is illegal
                                    where you live or where the service is
                                    operated;
                                </Trans>
                            </li>
                            <li>
                                <Trans>
                                    share material you have no right to share,
                                    including work protected by a copyright that
                                    belongs to someone else;
                                </Trans>
                            </li>
                            <li>
                                <Trans>
                                    distribute malware, or use a share link as a
                                    delivery channel for an attack;
                                </Trans>
                            </li>
                            <li>
                                <Trans>
                                    attempt to reach files, folders or accounts
                                    that were not shared with you;
                                </Trans>
                            </li>
                            <li>
                                <Trans>
                                    place a load on the service that degrades it
                                    for other users, whether through automation
                                    or otherwise.
                                </Trans>
                            </li>
                        </ul>
                        <p>
                            <Trans>
                                We may suspend an account that breaks these
                                rules, and will tell the account holder why
                                whenever the law allows us to.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>
                                5. Storage, compression and encryption
                            </Trans>
                        </h2>
                        <p>
                            <Trans>
                                When you upload a file, Driftbox compresses it
                                and then encrypts it before writing it to
                                storage. The compressed, encrypted form is the
                                only form ever stored; the readable version is
                                rebuilt when you, or someone you shared it with,
                                asks for it. Previews and thumbnails are
                                generated at upload time and stored the same
                                way.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                This is a technical demonstration. There is no
                                service level agreement, no uptime commitment
                                and no guaranteed backup. Keep your own copy of
                                anything that matters, and do not use Driftbox
                                as the only home of important data.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>6. Retention and deletion</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Files you delete are removed from the browsable
                                listing straight away and purged from storage
                                within thirty days. Deleting your account
                                removes your profile, your files and every share
                                you granted; shares that other people granted to
                                you simply stop being visible to you.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                We keep technical logs, which may contain your
                                account identifier and IP address, for twelve
                                months, and we may retain what an applicable law
                                requires us to retain for longer.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>7. Limitation of liability</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Driftbox is provided as is, without warranty of
                                any kind. To the fullest extent permitted by
                                law, Driftbox SAS is not liable for lost data,
                                lost profits, interrupted business or any
                                indirect damage arising from the use of, or the
                                inability to use, the service.
                            </Trans>
                        </p>
                        <p>
                            <Trans>
                                Nothing in this section limits liability that
                                cannot be limited under the applicable law, such
                                as liability for gross negligence or for
                                personal injury.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>8. Changes to these terms</Trans>
                        </h2>
                        <p>
                            <Trans>
                                We may update these terms as the service
                                changes. When a change is significant, we will
                                announce it in the application at least thirty
                                days before it takes effect. Continuing to use
                                Driftbox after that date means you accept the
                                updated terms; if you do not, you may delete
                                your account at any time.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>9. Governing law</Trans>
                        </h2>
                        <p>
                            <Trans>
                                These terms are governed by the laws of the
                                placeholder jurisdiction of France, and any
                                dispute that cannot be settled amicably will be
                                brought before the courts of Paris. This is a
                                placeholder clause in a fictional document and
                                has no legal effect.
                            </Trans>
                        </p>
                    </section>

                    <section className={styles.section}>
                        <h2>
                            <Trans>10. Contact</Trans>
                        </h2>
                        <p>
                            <Trans>
                                Questions about these terms go to
                                legal@driftbox.example, or by post to Driftbox
                                SAS, 12 rue des Nuages, 75000 Paris, France.
                                Both the address and the mailbox are fictional
                                and nobody reads them.
                            </Trans>
                        </p>
                    </section>
                </div>
            </article>
        </>
    )
}
