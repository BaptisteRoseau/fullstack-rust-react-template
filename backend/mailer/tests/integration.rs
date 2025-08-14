use mailer::*;

#[test]
fn builder() {
    let _mailer = Mailer::new(
        "smtp.server.com",
        "smtp_username",
        "smtp_password",
        Mailbox {
            name: "John Doe",
            email: "john.doe@server.com",
        },
        None,
    );
    _mailer.set_footer("Visit our website");
    let builder = _mailer.template(Template::new(
        Archetype::PlainHtml {
            title: "Mail title",
            body: "Mail body",
        },
        Language::English,
    ));
    let mail = builder.build();
    mail.unwrap().send();
}
