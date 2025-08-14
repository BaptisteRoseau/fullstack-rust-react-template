use std::rc::Rc;

use crate::templates::Template;

use lettre::error::Error;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MessageBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

//// =======================

pub struct Email {
    mailer: SmtpTransport,
    email: Message,
}

impl Email {
    pub fn new(mailer: SmtpTransport, email: Message) -> Self {
        Self { mailer, email }
    }

    pub fn send(&self) -> Result<(), Error> {
        Ok(self.mailer.send(&self.email)?)
    }
}

pub struct EmailBuilder {
    mailer: SmtpTransport,
    builder: Rc<MessageBuilder>,
    template: Template,
}

impl EmailBuilder {
    fn new(
        mailer: SmtpTransport,
        builder: Rc<MessageBuilder>,
        template: Template,
    ) -> Self {
        Self {
            mailer,
            builder,
            template,
        }
    }

    pub fn build(&self) -> Result<Email, Error> {
        let message_content = self.template.content();
        let email = self
            .builder
            .subject(message_content.title)
            .header(ContentType::TEXT_HTML)
            .body(message_content.body)?;
        Ok(Email::new(self.mailer.clone(), email))
    }
}

pub struct Mailer {
    mailer: SmtpTransport,
    footer: Option<String>,
    builder: Rc<MessageBuilder>,
}

impl Mailer {
    pub fn new(
        server: &str,
        username: &str,
        password: &str,
        from: Mailbox,
        reply_to: Option<Mailbox>,
    ) -> Self {
        let mailer = SmtpTransport::relay(server)
            .unwrap()
            .credentials(Credentials::new(username.to_owned(), password.to_owned()))
            .build();
        let mut builder = MessageBuilder::new().from(from);
        if let Some(reply_to) = reply_to {
            builder = builder.reply_to(reply_to);
        }
        Self {
            mailer,
            builder: Rc::new(builder),
            footer: None,
        }
    }

    pub fn set_footer(&mut self, footer_html: &str) -> &mut Self {
        self.footer = Some(footer_html.to_owned());
        self
    }

    pub fn template(&self, template: Template) -> EmailBuilder {
        EmailBuilder::new(self.builder.clone(), template)
    }
}
