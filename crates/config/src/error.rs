use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigParsingError {
    #[error("'{0}' does not exist")]
    NotFound(String),
    #[error("MissingPemPubCert")]
    MissingPemPubCert,
    #[error("MissingPemPrivKey")]
    MissingPemPrivKey,
    #[error("DefaultPasswordSaltInReleaseMode")]
    DefaultPasswordSaltInReleaseMode,
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("Cannot parse the given YAML file")]
    ParsingError(#[from] serde_yaml::Error),
}
