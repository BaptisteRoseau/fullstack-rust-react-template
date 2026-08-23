# Mailer

Outgoing email, built on [`lettre`](https://docs.rs/lettre).

**Not implemented.** [`src/lib.rs`](src/lib.rs) exports nothing: every module is commented
out, and `Email::send` and `EmailBuilder::build` are `todo!()`. Nothing in the workspace
depends on this crate yet.

## Intended shape

- [`builder.rs`](src/builder.rs) — `Mailer` (holds the SMTP transport and the sender
  mailbox), `EmailBuilder` (fills in a template) and `Email` (ready to send).
- [`templates.rs`](src/templates.rs) — `Template` picks an `Archetype` and a `Language`;
  `Archetype::content` renders the title and HTML body for that language.
- [`error.rs`](src/error.rs) — `Error`, wrapping `lettre::error::Error`.
