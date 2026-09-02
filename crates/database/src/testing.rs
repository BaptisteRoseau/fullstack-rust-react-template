//! In-memory [`Database`] test double, shared by downstream crates' tests.
//!
//! Enable via the `test-utils` feature (add it under `[dev-dependencies]`, not
//! `[dependencies]`, so it never leaks into non-test builds).

use std::{cmp::Reverse, collections::HashMap};

use async_trait::async_trait;
use uuid::Uuid;

use crate::database::{
    DatabaseApiKey, DatabaseDirectory, DatabaseDirectoryPermission, DatabaseFile,
    DatabaseFilePermission, DatabaseUser,
};
use crate::error::DatabaseError;
use crate::models::{
    ApiKey, Directory, DirectoryPermission, File, FilePermission, NewFile, User,
    UserPatch,
};

/// In-memory `Database` backed by plain `HashMap`s. Seed state through the
/// public fields (e.g. `MockDatabase { api_keys_by_hash, ..Default::default() }`)
/// before exercising the code under test.
///
/// `create_user` is intentionally left unimplemented: the real `Postgres`
/// implementation is currently broken (it never inserts), so a working mock
/// here would silently diverge from production behavior.
#[derive(Default)]
pub struct MockDatabase {
    pub users: HashMap<Uuid, User>,
    pub api_keys_by_id: HashMap<Uuid, ApiKey>,
    pub api_keys_by_hash: HashMap<String, ApiKey>,
    pub directories: HashMap<Uuid, Directory>,
    pub files: HashMap<Uuid, File>,
    /// Keyed by `(directory_id, grantee)`, mirroring the table's unique constraint.
    pub directory_permissions: HashMap<(Uuid, Uuid), DirectoryPermission>,
    /// Keyed by `(file_id, grantee)`, mirroring the table's unique constraint.
    pub file_permissions: HashMap<(Uuid, Uuid), FilePermission>,
}

#[async_trait]
impl DatabaseUser for MockDatabase {
    async fn create_user(
        &mut self,
        _patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        unimplemented!("create_user is broken upstream; see Postgres::create_user")
    }

    async fn update_user(
        &mut self,
        patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        let user = self
            .users
            .get_mut(&patch.id)
            .ok_or_else(|| Box::new(DatabaseError::NotFound(patch.id.to_string())))?;

        if let Some(username) = patch.username {
            user.username = username;
        }
        if let Some(first_name) = patch.first_name {
            user.first_name = first_name;
        }
        if let Some(last_name) = patch.last_name {
            user.last_name = last_name;
        }
        if let Some(email) = patch.email {
            user.email = email;
        }
        if let Some(permissions) = patch.permissions {
            user.permissions = permissions;
        }
        user.updated_at = chrono::Utc::now();

        Ok(user.clone())
    }

    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>> {
        self.users
            .get(&uuid)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(uuid.to_string())))
    }

    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>> {
        Ok(self.users.remove(&uuid).is_some())
    }

    async fn register(
        &mut self,
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Result<User, Box<DatabaseError>> {
        let now = chrono::Utc::now();
        let user = self.users.entry(id).or_insert_with(|| User {
            id,
            username: username.clone(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            email: email.clone(),
            permissions: serde_json::Value::Array(Vec::new()),
            created_at: now,
            updated_at: now,
        });

        if user.username != username || user.email != email {
            user.username = username;
            user.email = email;
            user.updated_at = now;
        }

        Ok(user.clone())
    }
}

#[async_trait]
impl DatabaseApiKey for MockDatabase {
    async fn create_api_key(
        &mut self,
        owner: Uuid,
        name: String,
        hash: String,
        permissions: serde_json::Value,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        if self.api_keys_by_hash.contains_key(&hash) {
            return Err(Box::new(DatabaseError::HashCollision));
        }

        let now = chrono::Utc::now();
        let key = ApiKey {
            id: Uuid::new_v4(),
            hash: hash.clone(),
            name,
            owner,
            permissions,
            created_at: now,
            updated_at: now,
        };
        self.api_keys_by_hash.insert(hash, key.clone());
        self.api_keys_by_id.insert(key.id, key.clone());
        Ok(key)
    }

    async fn read_api_key_by_id(&self, id: Uuid) -> Result<ApiKey, Box<DatabaseError>> {
        self.api_keys_by_id
            .get(&id)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(id.to_string())))
    }

    async fn read_api_key_by_hash(
        &self,
        hash: &str,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        self.api_keys_by_hash
            .get(hash)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(hash.to_string())))
    }

    async fn read_api_keys_by_owner(
        &self,
        owner: Uuid,
    ) -> Result<Vec<ApiKey>, Box<DatabaseError>> {
        let mut keys: Vec<ApiKey> = self
            .api_keys_by_id
            .values()
            .filter(|key| key.owner == owner)
            .cloned()
            .collect();
        keys.sort_by_key(|key| Reverse(key.created_at));
        Ok(keys)
    }

    async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        let Some(key) = self.api_keys_by_id.remove(&id) else {
            return Ok(false);
        };
        self.api_keys_by_hash.remove(&key.hash);
        Ok(true)
    }
}

#[async_trait]
impl DatabaseDirectory for MockDatabase {
    async fn create_directory(
        &mut self,
        owner: Uuid,
        parent_id: Option<Uuid>,
        name: String,
    ) -> Result<Directory, Box<DatabaseError>> {
        let now = chrono::Utc::now();
        let directory = Directory {
            id: Uuid::new_v4(),
            owner,
            parent_id,
            name,
            created_at: now,
            updated_at: now,
        };
        self.directories.insert(directory.id, directory.clone());
        Ok(directory)
    }

    async fn read_directory(&self, id: Uuid) -> Result<Directory, Box<DatabaseError>> {
        self.directories
            .get(&id)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(id.to_string())))
    }

    async fn read_child_directories(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>> {
        Ok(sorted_by_name(
            self.directories
                .values()
                .filter(|directory| directory.parent_id == Some(parent_id))
                .cloned(),
        ))
    }

    async fn read_root_directories(
        &self,
        owner: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>> {
        Ok(sorted_by_name(
            self.directories
                .values()
                .filter(|directory| {
                    directory.owner == owner && directory.parent_id.is_none()
                })
                .cloned(),
        ))
    }

    async fn read_directory_ancestors(
        &self,
        id: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>> {
        let mut chain = Vec::new();
        let mut current = Some(id);
        while let Some(directory_id) = current {
            let Some(directory) = self.directories.get(&directory_id) else {
                break;
            };
            chain.push(directory.clone());
            current = directory.parent_id;
        }
        Ok(chain)
    }

    async fn update_directory(
        &mut self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Option<Uuid>>,
    ) -> Result<Directory, Box<DatabaseError>> {
        let directory = self
            .directories
            .get_mut(&id)
            .ok_or_else(|| Box::new(DatabaseError::NotFound(id.to_string())))?;

        if let Some(name) = name {
            directory.name = name;
        }
        if let Some(parent_id) = parent_id {
            directory.parent_id = parent_id;
        }
        directory.updated_at = chrono::Utc::now();

        Ok(directory.clone())
    }

    async fn delete_directory(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        if !self.directories.contains_key(&id) {
            return Ok(false);
        }
        for doomed in self.descendant_directory_ids(id) {
            self.directories.remove(&doomed);
            self.directory_permissions
                .retain(|(directory_id, _), _| *directory_id != doomed);
            let orphaned: Vec<Uuid> = self
                .files
                .values()
                .filter(|file| file.parent_id == Some(doomed))
                .map(|file| file.id)
                .collect();
            for file_id in orphaned {
                self.files.remove(&file_id);
                self.file_permissions.retain(|(id, _), _| *id != file_id);
            }
        }
        Ok(true)
    }
}

#[async_trait]
impl DatabaseFile for MockDatabase {
    async fn create_file(&mut self, file: NewFile) -> Result<File, Box<DatabaseError>> {
        let now = chrono::Utc::now();
        let file = File {
            id: file.id,
            owner: file.owner,
            parent_id: file.parent_id,
            name: file.name,
            storage_key: file.storage_key,
            mime_type: file.mime_type,
            size_bytes: file.size_bytes,
            stored_size_bytes: file.stored_size_bytes,
            is_compressed: file.is_compressed,
            encrypted_dek: file.encrypted_dek,
            dek_nonce: file.dek_nonce,
            content_nonce: file.content_nonce,
            thumbnail_storage_key: file.thumbnail_storage_key,
            thumbnail_nonce: file.thumbnail_nonce,
            created_at: now,
            updated_at: now,
        };
        self.files.insert(file.id, file.clone());
        Ok(file)
    }

    async fn read_file(&self, id: Uuid) -> Result<File, Box<DatabaseError>> {
        self.files
            .get(&id)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(id.to_string())))
    }

    async fn read_files_by_parent(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<File>, Box<DatabaseError>> {
        Ok(sorted_by_name(
            self.files
                .values()
                .filter(|file| file.parent_id == Some(parent_id))
                .cloned(),
        ))
    }

    async fn read_root_files(
        &self,
        owner: Uuid,
    ) -> Result<Vec<File>, Box<DatabaseError>> {
        Ok(sorted_by_name(
            self.files
                .values()
                .filter(|file| file.owner == owner && file.parent_id.is_none())
                .cloned(),
        ))
    }

    async fn update_file(
        &mut self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Option<Uuid>>,
    ) -> Result<File, Box<DatabaseError>> {
        let file = self
            .files
            .get_mut(&id)
            .ok_or_else(|| Box::new(DatabaseError::NotFound(id.to_string())))?;

        if let Some(name) = name {
            file.name = name;
        }
        if let Some(parent_id) = parent_id {
            file.parent_id = parent_id;
        }
        file.updated_at = chrono::Utc::now();

        Ok(file.clone())
    }

    async fn delete_file(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        if self.files.remove(&id).is_none() {
            return Ok(false);
        }
        self.file_permissions
            .retain(|(file_id, _), _| *file_id != id);
        Ok(true)
    }
}

#[async_trait]
impl DatabaseDirectoryPermission for MockDatabase {
    async fn upsert_directory_permission(
        &mut self,
        directory_id: Uuid,
        grantee: Uuid,
        permission_level: &str,
        granted_by: Uuid,
    ) -> Result<DirectoryPermission, Box<DatabaseError>> {
        let now = chrono::Utc::now();
        let permission = self
            .directory_permissions
            .entry((directory_id, grantee))
            .and_modify(|permission| {
                permission.permission_level = permission_level.to_string();
                permission.granted_by = granted_by;
                permission.updated_at = now;
            })
            .or_insert_with(|| DirectoryPermission {
                id: Uuid::new_v4(),
                directory_id,
                grantee,
                permission_level: permission_level.to_string(),
                granted_by,
                created_at: now,
                updated_at: now,
            });
        Ok(permission.clone())
    }

    async fn read_directory_permission(
        &self,
        directory_id: Uuid,
        grantee: Uuid,
    ) -> Result<Option<DirectoryPermission>, Box<DatabaseError>> {
        Ok(self
            .directory_permissions
            .get(&(directory_id, grantee))
            .cloned())
    }

    async fn read_directory_permissions_for_grantee(
        &self,
        directory_ids: &[Uuid],
        grantee: Uuid,
    ) -> Result<Vec<DirectoryPermission>, Box<DatabaseError>> {
        Ok(directory_ids
            .iter()
            .filter_map(|id| self.directory_permissions.get(&(*id, grantee)).cloned())
            .collect())
    }

    async fn read_directory_permissions(
        &self,
        directory_id: Uuid,
    ) -> Result<Vec<DirectoryPermission>, Box<DatabaseError>> {
        let mut permissions: Vec<DirectoryPermission> = self
            .directory_permissions
            .values()
            .filter(|permission| permission.directory_id == directory_id)
            .cloned()
            .collect();
        permissions.sort_by_key(|permission| permission.created_at);
        Ok(permissions)
    }

    async fn delete_directory_permission(
        &mut self,
        directory_id: Uuid,
        grantee: Uuid,
    ) -> Result<bool, Box<DatabaseError>> {
        Ok(self
            .directory_permissions
            .remove(&(directory_id, grantee))
            .is_some())
    }
}

#[async_trait]
impl DatabaseFilePermission for MockDatabase {
    async fn upsert_file_permission(
        &mut self,
        file_id: Uuid,
        grantee: Uuid,
        permission_level: &str,
        granted_by: Uuid,
    ) -> Result<FilePermission, Box<DatabaseError>> {
        let now = chrono::Utc::now();
        let permission = self
            .file_permissions
            .entry((file_id, grantee))
            .and_modify(|permission| {
                permission.permission_level = permission_level.to_string();
                permission.granted_by = granted_by;
                permission.updated_at = now;
            })
            .or_insert_with(|| FilePermission {
                id: Uuid::new_v4(),
                file_id,
                grantee,
                permission_level: permission_level.to_string(),
                granted_by,
                created_at: now,
                updated_at: now,
            });
        Ok(permission.clone())
    }

    async fn read_file_permission(
        &self,
        file_id: Uuid,
        grantee: Uuid,
    ) -> Result<Option<FilePermission>, Box<DatabaseError>> {
        Ok(self.file_permissions.get(&(file_id, grantee)).cloned())
    }

    async fn read_file_permissions(
        &self,
        file_id: Uuid,
    ) -> Result<Vec<FilePermission>, Box<DatabaseError>> {
        let mut permissions: Vec<FilePermission> = self
            .file_permissions
            .values()
            .filter(|permission| permission.file_id == file_id)
            .cloned()
            .collect();
        permissions.sort_by_key(|permission| permission.created_at);
        Ok(permissions)
    }

    async fn delete_file_permission(
        &mut self,
        file_id: Uuid,
        grantee: Uuid,
    ) -> Result<bool, Box<DatabaseError>> {
        Ok(self.file_permissions.remove(&(file_id, grantee)).is_some())
    }
}

impl MockDatabase {
    /// `id` and every directory below it, emulating the `ON DELETE CASCADE`
    /// Postgres applies to the self-referencing `parent_id` foreign key.
    fn descendant_directory_ids(&self, id: Uuid) -> Vec<Uuid> {
        let mut found = vec![id];
        let mut cursor = 0;
        while cursor < found.len() {
            let parent = found[cursor];
            cursor += 1;
            found.extend(
                self.directories
                    .values()
                    .filter(|directory| directory.parent_id == Some(parent))
                    .map(|directory| directory.id),
            );
        }
        found
    }
}

/// Postgres orders these listings by name; the maps behind the double do not,
/// so the same ordering is applied here to keep both sides of the contract equal.
fn sorted_by_name<T: Named>(rows: impl Iterator<Item = T>) -> Vec<T> {
    let mut rows: Vec<T> = rows.collect();
    rows.sort_by(|left, right| left.name().cmp(right.name()));
    rows
}

trait Named {
    fn name(&self) -> &str;
}

impl Named for Directory {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for File {
    fn name(&self) -> &str {
        &self.name
    }
}
