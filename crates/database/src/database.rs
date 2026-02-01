use crate::crud::Crud;

/**
* Defines the Database trait interface.
**/
use super::error::DatabaseError;
use async_trait::async_trait;
use config::Config;

#[async_trait]
pub trait Database {
    fn close(&mut self) -> Result<(), DatabaseError>;
    fn init(&mut self, config: &Config) -> Result<&mut Self, DatabaseError>;
    fn create<T>(&mut self, item: &dyn Crud) -> Result<&mut T, DatabaseError>;
    fn read_by_id<T>(&mut self, item: &dyn Crud) -> Result<&mut T, DatabaseError>;
    fn update(&mut self, item: &dyn Crud) -> Result<&mut Self, DatabaseError>;
    fn delete_by_id(&mut self, item: &dyn Crud) -> Result<&mut Self, DatabaseError>;
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
