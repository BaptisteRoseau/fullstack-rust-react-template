use mailer::*;

#[test]
fn builder() {
    let mut _mailer = Mailer::new(
        "smtp.server.com",
        "smtp_username",
        "smtp_password",
        Mailbox {
            name: Some("John Doe".into()),
            email: "john.doe@server.com".into(),
        },
        None,
    );
    _mailer.set_footer("Visit our website");
    let builder = _mailer.template(Template::new(
        Archetype::PlainHtml {
            title: "Mail title".to_owned(),
            body: "Mail body".to_owned(),
        },
        Some(Language::English),
    ));
    let mail = builder.build();
    mail.unwrap().send();
}
