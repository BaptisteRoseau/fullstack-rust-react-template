#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("An unknown error has occured")]
    Unknown,
}
