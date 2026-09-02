use crate::database::{
    DatabaseApiKey, DatabaseDirectory, DatabaseDirectoryPermission, DatabaseFile,
    DatabaseFilePermission, DatabaseUser,
};
use async_trait::async_trait;

/// Defines the Database trait interface.
#[async_trait]
pub trait Database:
    Send
    + Sync
    + DatabaseApiKey
    + DatabaseUser
    + DatabaseDirectory
    + DatabaseFile
    + DatabaseDirectoryPermission
    + DatabaseFilePermission
{
}

impl<
    T: Send
        + Sync
        + DatabaseApiKey
        + DatabaseUser
        + DatabaseDirectory
        + DatabaseFile
        + DatabaseDirectoryPermission
        + DatabaseFilePermission,
> Database for T
{
}
