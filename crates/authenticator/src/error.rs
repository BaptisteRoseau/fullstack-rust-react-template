use jsonwebtoken::errors::Error as JwtError;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticatorError {
    #[error("No JWK sent from the Auth provider")]
    NoJwk,
    #[error("Cannot read authentication server")]
    RequestError(#[from] reqwest::Error),
    #[error("The user's authentication failed")]
    AuthenticationFailure,
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),
    #[error("Invalid Realm: {0}")]
    InvalidRealm(String),
    #[error("OIDC error: {0}")]
    Oidc(String),
    /// The provider rejected the token: it was revoked (logout elsewhere,
    /// admin-terminated session) or is otherwise no longer accepted.
    #[error("The OIDC provider rejected the token")]
    OidcRejected,
    #[error("Invalid or expired login state")]
    InvalidState,
    #[error("{0}")]
    Message(String),
}

impl From<reqwest::Error> for Box<AuthenticatorError> {
    fn from(value: reqwest::Error) -> Self {
        Box::new(value.into())
    }
}

impl From<JwtError> for Box<AuthenticatorError> {
    fn from(value: JwtError) -> Self {
        Box::new(value.into())
    }
}

impl From<&str> for Box<AuthenticatorError> {
    fn from(value: &str) -> Self {
        Box::new(AuthenticatorError::Message(value.to_string()))
    }
}

/// Wraps a message into a boxed [`AuthenticatorError::Oidc`], matching the
/// error type used throughout the crate.
pub(crate) fn oidc_error(message: impl Into<String>) -> Box<AuthenticatorError> {
    Box::new(AuthenticatorError::Oidc(message.into()))
}

/// Renders an error together with its causes, `"outer: inner: root"`.
///
/// `reqwest::Error` displays as "error sending request for url (...)" and keeps
/// the interesting part — refused connection, DNS failure, TLS error — in its
/// source, so a bare `{e}` tells an operator nothing about what to fix.
pub(crate) fn error_chain(error: &dyn std::error::Error) -> String {
    let mut rendered = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        rendered.push_str(&format!(": {cause}"));
        source = cause.source();
    }
    rendered
}
