mod builder;
mod error;
mod templates;

pub use builder::{Email, EmailBuilder, Mailer};
pub use error::Error;
pub use templates::{Archetype, Language, Template};

pub use lettre::message::Mailbox;
