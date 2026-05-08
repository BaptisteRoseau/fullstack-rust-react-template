use jsonwebtoken::errors::Error as JwtError;

#[derive(thiserror::Error, Debug)]
pub enum AuthenticatorError {
    #[error("No JWK sent from the Auth provider")]
    NoJwk,
    #[error("The token has expired")]
    Expired,
    #[error("The token signature is invalid")]
    InvalidSignature,
    #[error("Cannot read authentication server")]
    RequestError(#[from] reqwest::Error),
    #[error("The user's authentication failed")]
    AuthenticationFailure,
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),
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
