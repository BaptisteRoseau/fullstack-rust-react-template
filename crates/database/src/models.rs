// Re-export generated models
pub use crate::generated_models::*;

impl ApiKey {
    pub fn id(&self) -> uuid::Uuid { self.id }
    pub fn owner(&self) -> uuid::Uuid { self.owner }
    pub fn name(&self) -> &str { &self.name }
    pub fn hash(&self) -> &str { &self.hash }
    pub fn permissions(&self) -> &String { &self.permissions }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created_at }
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> { self.updated_at }
}
