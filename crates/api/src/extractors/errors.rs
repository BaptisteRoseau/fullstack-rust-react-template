#[derive(Debug, thiserror::Error)]
pub enum ExtractorError {
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("User is not logged in")]
    NotLoggedIn,
    #[error("Unexpected Error")]
    Unexpected(#[from] anyhow::Error),
}
