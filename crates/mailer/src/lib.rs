mod builder;
mod errors;
mod templates;

pub use builder::{Email, EmailBuilder, Mailer};
pub use errors::Error;
pub use templates::{Archetype, Language, Template};

pub use lettre::message::Mailbox;
