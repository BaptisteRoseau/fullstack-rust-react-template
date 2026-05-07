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
    #[error("Unexpected Error")]
    Unexpected(#[from] Box<dyn std::error::Error>),
}

impl From<reqwest::Error> for Box<AuthenticatorError> {
    fn from(value: reqwest::Error) -> Self {
        Box::new(value.into())
    }
}
