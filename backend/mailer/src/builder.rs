use crate::templates::{Archetype, Language, MessageContent, Template};

use lettre::error::Error;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, Mailboxes, MessageBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {

    let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

//// =======================

pub struct Email {
    mailer: SmtpTransport,
    email: Message,
}

impl Email {
    pub fn new() -> Self {
        todo!()
    }

    pub fn send() -> Result<_> {
        todo!()
    }
}

struct EmailBuilder {
    footer: Option<String>,
    builder: MessageBuilder,
    template: Template,
    mailer: SmtpTransport
}

impl EmailBuilder {
    pub fn template(&mut self, template: Template) -> Self {
        Self {
            footer: None,
            builder: Message::builder(),
            template,
        }
    }

    pub fn credentials(&mut self) {
        let creds =
            Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

            let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    }

    pub fn build(&self, template: Template) -> Result<Email, Error> {
        let message_content = template.content();
        let email = self
            .builder
            .subject(message_content.title)
            .header(ContentType::TEXT_HTML)
            .body(message_content.body)?;
    }

    pub fn set_footer<T: ToString>(&mut self, footer: T) {}
}
