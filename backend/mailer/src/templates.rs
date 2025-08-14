/// A mail template. Used to generate the mail content based on the
/// language and mail archetype.
/// English by default.
#[derive(Clone)]
pub struct Template {
    language: Language,
    archetype: Archetype,
}

impl Template {
    pub fn new(archetype: Archetype, language: Option<Language>) -> Self {
        Self {
            language: language.unwrap_or(Language::English),
            archetype,
        }
    }

    pub fn content(&self) -> MessageContent {
        self.archetype.content(self.language)
    }
}

/// The language of the target email
#[derive(Copy, Clone)]
pub enum Language {
    English,
    French,
}

/// The generated email. Contains the email title and body.
pub struct MessageContent {
    pub title: String,
    pub body: String,
}

impl MessageContent {
    pub fn new<T: ToString, B: ToString>(title: T, body: B) -> Self {
        Self {
            title: title.to_string(),
            body: format!("<div>{}</div>", body.to_string()),
        }
    }
}

/// The archetype of the email. This is used to build templates.
/// Messages' body are written in HTML format.
#[derive(Clone)]
pub enum Archetype {
    PlainHtml { title: String, body: String },
}

impl Archetype {
    // Conventions:
    // Create a function per language, where the mail body and title is defined.
    // Define the mail body directly within their match arm.
    // Messages are written in HTML format.

    fn content(&self, language: Language) -> MessageContent {
        match language {
            Language::English => self.english(),
            Language::French => self.french(),
        }
    }

    fn english(&self) -> MessageContent {
        match self {
            Archetype::PlainHtml { title, body } => MessageContent::new(title, body),
        }
    }

    fn french(&self) -> MessageContent {
        match self {
            Archetype::PlainHtml { title, body } => MessageContent::new(title, body),
        }
    }
}
