#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Cannot read authentication server")]
    RequestError(#[from] reqwest::Error),
    #[error("Unexpected Error")]
    Unexpected(#[from] Box<dyn std::error::Error>),
}
