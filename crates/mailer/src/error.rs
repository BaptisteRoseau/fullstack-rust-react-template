#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LetttreError(#[from] lettre::error::Error),
}
