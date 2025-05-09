/**
* Defines the Database trait interface.
**/
use super::errors::DatabaseError;
use crate::config::Config;
use std::future::Future;

pub(crate) trait Database {
    fn close(&mut self) -> impl Future<Output = Result<(), DatabaseError>> + Send;
    fn init(
        &mut self,
        config: &Config,
    ) -> impl Future<Output = Result<&mut Self, DatabaseError>> + Send;
}

/* =============================================================================
TEST
============================================================================= */

#[cfg(test)]
mod test {
    // TODO: Write the tests only using the Database trait,
    // that way they will be re-usable for other implementations !
    // Do this only when the API for managing users/cards/companies is over
    // to avoid re-writing the tests unnecessarily.

    // Make it derivable to ease the testing of implementations
}
