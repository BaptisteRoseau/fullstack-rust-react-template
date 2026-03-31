#[derive(thiserror::Error, Debug)]
pub enum AuthenticatorError {
    #[error("The token has expired")]
    Expired,
    #[error("The token signature is invalid")]
    InvalidSignarture,
    #[error("Cannot read authentication server")]
    RequestError(#[from] reqwest::Error),
    #[error("The user's authentication failed")]
    AuthenticationFailure,
    #[error("Unexpected Error")]
    Unexpected(#[from] Box<dyn std::error::Error>),
}
