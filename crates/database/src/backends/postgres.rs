use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::crud::{CrudError, CrudExecutor, CrudValue};
#[warn(dead_code)]
use crate::database::{
    DatabaseApiKey, DatabaseDirectory, DatabaseDirectoryPermission, DatabaseFile,
    DatabaseFilePermission, DatabaseUser,
};
use crate::error::DatabaseError;
use crate::models::{
    ApiKey, Directory, DirectoryPermission, File, FilePermission, NewFile, User,
    UserPatch,
};
use config::Config;
use tracing::warn;

#[derive(Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn try_from(config: &Config) -> Result<Self, DatabaseError> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.postgres.user,
            config.postgres.password,
            config.postgres.host,
            config.postgres.port,
            config.postgres.database,
        );
        let pool = PgPoolOptions::new().max_connections(10).connect(&url).await;
        match pool {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => {
                warn!("Could not connect to database yet: {e}");
                let pool = PgPoolOptions::new()
                    .max_connections(10)
                    .connect_lazy(&url)?;
                Ok(Self { pool })
            }
        }
    }

    pub async fn try_from_url(url: &str) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// sqlx 0.8+ requires `&'static str` for prepared-statement caching.
// We intern each unique SQL string so it is leaked at most once per unique query,
// bounding total allocation to the (small, finite) set of distinct SQL strings used.
static SQL_INTERN: Mutex<Option<HashMap<String, &'static str>>> = Mutex::new(None);

fn intern_sql(sql: &str) -> &'static str {
    let mut guard = SQL_INTERN.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);
    if let Some(&s) = map.get(sql) {
        return s;
    }
    let leaked: &'static str = Box::leak(sql.to_string().into_boxed_str());
    map.insert(sql.to_string(), leaked);
    leaked
}

fn bind_crud_value_query_as<'q, T>(
    query: sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>,
    value: CrudValue,
) -> sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow>,
{
    match value {
        CrudValue::Uuid(v) => query.bind(v),
        CrudValue::String(v) => query.bind(v),
        CrudValue::OptionString(v) => query.bind(v),
        CrudValue::DateTime(v) => query.bind(v),
        CrudValue::OptionDateTime(v) => query.bind(v),
        CrudValue::Bool(v) => query.bind(v),
        CrudValue::OptionBool(v) => query.bind(v),
        CrudValue::I32(v) => query.bind(v),
        CrudValue::OptionI32(v) => query.bind(v),
        CrudValue::I64(v) => query.bind(v),
        CrudValue::OptionI64(v) => query.bind(v),
        CrudValue::F64(v) => query.bind(v),
        CrudValue::OptionF64(v) => query.bind(v),
        CrudValue::Json(v) => query.bind(v),
    }
}

fn bind_crud_value_query(
    query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    value: CrudValue,
) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
    match value {
        CrudValue::Uuid(v) => query.bind(v),
        CrudValue::String(v) => query.bind(v),
        CrudValue::OptionString(v) => query.bind(v),
        CrudValue::DateTime(v) => query.bind(v),
        CrudValue::OptionDateTime(v) => query.bind(v),
        CrudValue::Bool(v) => query.bind(v),
        CrudValue::OptionBool(v) => query.bind(v),
        CrudValue::I32(v) => query.bind(v),
        CrudValue::OptionI32(v) => query.bind(v),
        CrudValue::I64(v) => query.bind(v),
        CrudValue::OptionI64(v) => query.bind(v),
        CrudValue::F64(v) => query.bind(v),
        CrudValue::OptionF64(v) => query.bind(v),
        CrudValue::Json(v) => query.bind(v),
    }
}

#[async_trait]
impl CrudExecutor for Postgres {
    async fn crud_fetch_one<T>(
        &self,
        query: &str,
        args: Vec<CrudValue>,
    ) -> Result<T, CrudError>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let mut q = sqlx::query_as::<_, T>(intern_sql(query));
        for arg in args {
            q = bind_crud_value_query_as(q, arg);
        }
        Ok(q.fetch_one(&self.pool).await?)
    }

    async fn crud_fetch_all<T>(
        &self,
        query: &str,
        args: Vec<CrudValue>,
    ) -> Result<Vec<T>, CrudError>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let mut q = sqlx::query_as::<_, T>(intern_sql(query));
        for arg in args {
            q = bind_crud_value_query_as(q, arg);
        }
        Ok(q.fetch_all(&self.pool).await?)
    }

    async fn crud_execute(
        &self,
        query: &str,
        args: Vec<CrudValue>,
    ) -> Result<u64, CrudError> {
        let mut q = sqlx::query(intern_sql(query));
        for arg in args {
            q = bind_crud_value_query(q, arg);
        }
        Ok(q.execute(&self.pool).await?.rows_affected())
    }
}

#[async_trait]
impl DatabaseUser for Postgres {
    async fn create_user(
        &mut self,
        patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        Ok(patch.execute(self).await?)
    }
    async fn update_user(
        &mut self,
        patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        let id = patch.id;
        patch.execute(self).await.map_err(|e| match e {
            CrudError::Sqlx(sqlx::Error::RowNotFound) => {
                Box::new(DatabaseError::NotFound(id.to_string()))
            }
            other => Box::new(DatabaseError::Crud(other)),
        })
    }
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Box::new(DatabaseError::NotFound(uuid.to_string()))
                }
                other => Box::new(DatabaseError::Sqlx(other)),
            })
    }
    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>> {
        let q = sqlx::query("DELETE * FROM user where id == %s").bind(uuid);
        Ok(q.execute(&self.pool).await?.rows_affected() == 1)
    }

    async fn register(
        &mut self,
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Result<User, Box<DatabaseError>> {
        let existing = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let user = match existing {
            None => {
                sqlx::query_as::<_, User>(
                    "INSERT INTO users (id, username, first_name, last_name, email) \
                     VALUES ($1, $2, $3, $4, $5) RETURNING *",
                )
                .bind(id)
                .bind(username)
                .bind(first_name)
                .bind(last_name)
                .bind(email)
                .fetch_one(&self.pool)
                .await?
            }
            Some(current) if current.username == username && current.email == email => {
                current
            }
            Some(_) => {
                sqlx::query_as::<_, User>(
                    "UPDATE users SET username = $1, email = $2 WHERE id = $3 \
                     RETURNING *",
                )
                .bind(username)
                .bind(email)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(user)
    }
}

#[async_trait]
impl DatabaseApiKey for Postgres {
    async fn create_api_key(
        &mut self,
        owner: Uuid,
        name: String,
        hash: String,
        permissions: serde_json::Value,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        let result = sqlx::query_as::<_, ApiKey>(
            "INSERT INTO api_key (owner, name, hash, permissions) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(owner)
        .bind(name)
        .bind(hash)
        .bind(permissions)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(key) => Ok(key),
            Err(sqlx::Error::Database(db_err))
                if db_err.constraint() == Some("api_key_hash_key") =>
            {
                Err(Box::new(DatabaseError::HashCollision))
            }
            Err(e) => Err(Box::new(DatabaseError::Sqlx(e))),
        }
    }

    async fn read_api_key_by_id(&self, id: Uuid) -> Result<ApiKey, Box<DatabaseError>> {
        sqlx::query_as::<_, ApiKey>("SELECT * FROM api_key WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Box::new(DatabaseError::NotFound(id.to_string()))
                }
                other => Box::new(DatabaseError::Sqlx(other)),
            })
    }

    async fn read_api_key_by_hash(
        &self,
        hash: &str,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        sqlx::query_as::<_, ApiKey>("SELECT * FROM api_key WHERE hash = $1")
            .bind(hash)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Box::new(DatabaseError::NotFound("api_key".to_string()))
                }
                other => Box::new(DatabaseError::Sqlx(other)),
            })
    }

    async fn read_api_keys_by_owner(
        &self,
        owner: Uuid,
    ) -> Result<Vec<ApiKey>, Box<DatabaseError>> {
        sqlx::query_as::<_, ApiKey>(
            "SELECT * FROM api_key WHERE owner = $1 ORDER BY created_at DESC",
        )
        .bind(owner)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        let rows = sqlx::query("DELETE FROM api_key WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(DatabaseError::Sqlx(e)))?;
        Ok(rows.rows_affected() == 1)
    }
}

/// Maps a `RowNotFound` onto [`DatabaseError::NotFound`] carrying the id that
/// was looked up, leaving every other sqlx failure as-is.
fn not_found_on_missing_row(error: sqlx::Error, id: Uuid) -> Box<DatabaseError> {
    match error {
        sqlx::Error::RowNotFound => Box::new(DatabaseError::NotFound(id.to_string())),
        other => Box::new(DatabaseError::Sqlx(other)),
    }
}

#[async_trait]
impl DatabaseDirectory for Postgres {
    async fn create_directory(
        &mut self,
        owner: Uuid,
        parent_id: Option<Uuid>,
        name: String,
    ) -> Result<Directory, Box<DatabaseError>> {
        sqlx::query_as::<_, Directory>(
            "INSERT INTO directories (owner, parent_id, name) VALUES ($1, $2, $3) \
             RETURNING *",
        )
        .bind(owner)
        .bind(parent_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_directory(&self, id: Uuid) -> Result<Directory, Box<DatabaseError>> {
        sqlx::query_as::<_, Directory>("SELECT * FROM directories WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| not_found_on_missing_row(e, id))
    }

    async fn read_child_directories(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>> {
        sqlx::query_as::<_, Directory>(
            "SELECT * FROM directories WHERE parent_id = $1 ORDER BY name ASC",
        )
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_root_directories(
        &self,
        owner: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>> {
        sqlx::query_as::<_, Directory>(
            "SELECT * FROM directories WHERE owner = $1 AND parent_id IS NULL \
             ORDER BY name ASC",
        )
        .bind(owner)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_directory_ancestors(
        &self,
        id: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>> {
        sqlx::query_as::<_, Directory>(
            "WITH RECURSIVE ancestors AS ( \
                 SELECT directories.*, 0 AS depth FROM directories WHERE id = $1 \
               UNION ALL \
                 SELECT parent.*, child.depth + 1 \
                 FROM directories parent \
                 JOIN ancestors child ON parent.id = child.parent_id \
             ) SELECT id, owner, parent_id, name, created_at, updated_at \
             FROM ancestors ORDER BY depth ASC",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn update_directory(
        &mut self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Option<Uuid>>,
    ) -> Result<Directory, Box<DatabaseError>> {
        let moves = parent_id.is_some();
        sqlx::query_as::<_, Directory>(
            "UPDATE directories SET name = COALESCE($2, name), \
             parent_id = CASE WHEN $3 THEN $4 ELSE parent_id END \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(moves)
        .bind(parent_id.flatten())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| not_found_on_missing_row(e, id))
    }

    async fn delete_directory(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        let rows = sqlx::query("DELETE FROM directories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(DatabaseError::Sqlx(e)))?;
        Ok(rows.rows_affected() == 1)
    }
}

#[async_trait]
impl DatabaseFile for Postgres {
    async fn create_file(&mut self, file: NewFile) -> Result<File, Box<DatabaseError>> {
        sqlx::query_as::<_, File>(
            "INSERT INTO files (id, owner, parent_id, name, storage_key, mime_type, \
             size_bytes, stored_size_bytes, is_compressed, encrypted_dek, dek_nonce, \
             content_nonce, thumbnail_storage_key, thumbnail_nonce) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) \
             RETURNING *",
        )
        .bind(file.id)
        .bind(file.owner)
        .bind(file.parent_id)
        .bind(file.name)
        .bind(file.storage_key)
        .bind(file.mime_type)
        .bind(file.size_bytes)
        .bind(file.stored_size_bytes)
        .bind(file.is_compressed)
        .bind(file.encrypted_dek)
        .bind(file.dek_nonce)
        .bind(file.content_nonce)
        .bind(file.thumbnail_storage_key)
        .bind(file.thumbnail_nonce)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_file(&self, id: Uuid) -> Result<File, Box<DatabaseError>> {
        sqlx::query_as::<_, File>("SELECT * FROM files WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| not_found_on_missing_row(e, id))
    }

    async fn read_files_by_parent(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<File>, Box<DatabaseError>> {
        sqlx::query_as::<_, File>(
            "SELECT * FROM files WHERE parent_id = $1 ORDER BY name ASC",
        )
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_root_files(
        &self,
        owner: Uuid,
    ) -> Result<Vec<File>, Box<DatabaseError>> {
        sqlx::query_as::<_, File>(
            "SELECT * FROM files WHERE owner = $1 AND parent_id IS NULL \
             ORDER BY name ASC",
        )
        .bind(owner)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn update_file(
        &mut self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Option<Uuid>>,
    ) -> Result<File, Box<DatabaseError>> {
        let moves = parent_id.is_some();
        sqlx::query_as::<_, File>(
            "UPDATE files SET name = COALESCE($2, name), \
             parent_id = CASE WHEN $3 THEN $4 ELSE parent_id END \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(moves)
        .bind(parent_id.flatten())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| not_found_on_missing_row(e, id))
    }

    async fn delete_file(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        let rows = sqlx::query("DELETE FROM files WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(DatabaseError::Sqlx(e)))?;
        Ok(rows.rows_affected() == 1)
    }
}

#[async_trait]
impl DatabaseDirectoryPermission for Postgres {
    async fn upsert_directory_permission(
        &mut self,
        directory_id: Uuid,
        grantee: Uuid,
        permission_level: &str,
        granted_by: Uuid,
    ) -> Result<DirectoryPermission, Box<DatabaseError>> {
        sqlx::query_as::<_, DirectoryPermission>(
            "INSERT INTO directory_permissions \
             (directory_id, grantee, permission_level, granted_by) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT ON CONSTRAINT uq__directory_permissions DO UPDATE SET \
             permission_level = EXCLUDED.permission_level, \
             granted_by = EXCLUDED.granted_by \
             RETURNING *",
        )
        .bind(directory_id)
        .bind(grantee)
        .bind(permission_level)
        .bind(granted_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_directory_permission(
        &self,
        directory_id: Uuid,
        grantee: Uuid,
    ) -> Result<Option<DirectoryPermission>, Box<DatabaseError>> {
        sqlx::query_as::<_, DirectoryPermission>(
            "SELECT * FROM directory_permissions WHERE directory_id = $1 \
             AND grantee = $2",
        )
        .bind(directory_id)
        .bind(grantee)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_directory_permissions_for_grantee(
        &self,
        directory_ids: &[Uuid],
        grantee: Uuid,
    ) -> Result<Vec<DirectoryPermission>, Box<DatabaseError>> {
        sqlx::query_as::<_, DirectoryPermission>(
            "SELECT * FROM directory_permissions WHERE directory_id = ANY($1) \
             AND grantee = $2",
        )
        .bind(directory_ids)
        .bind(grantee)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_directory_permissions(
        &self,
        directory_id: Uuid,
    ) -> Result<Vec<DirectoryPermission>, Box<DatabaseError>> {
        sqlx::query_as::<_, DirectoryPermission>(
            "SELECT * FROM directory_permissions WHERE directory_id = $1 \
             ORDER BY created_at ASC",
        )
        .bind(directory_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn delete_directory_permission(
        &mut self,
        directory_id: Uuid,
        grantee: Uuid,
    ) -> Result<bool, Box<DatabaseError>> {
        let rows = sqlx::query(
            "DELETE FROM directory_permissions WHERE directory_id = $1 AND grantee = $2",
        )
        .bind(directory_id)
        .bind(grantee)
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))?;
        Ok(rows.rows_affected() == 1)
    }
}

#[async_trait]
impl DatabaseFilePermission for Postgres {
    async fn upsert_file_permission(
        &mut self,
        file_id: Uuid,
        grantee: Uuid,
        permission_level: &str,
        granted_by: Uuid,
    ) -> Result<FilePermission, Box<DatabaseError>> {
        sqlx::query_as::<_, FilePermission>(
            "INSERT INTO file_permissions \
             (file_id, grantee, permission_level, granted_by) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT ON CONSTRAINT uq__file_permissions DO UPDATE SET \
             permission_level = EXCLUDED.permission_level, \
             granted_by = EXCLUDED.granted_by \
             RETURNING *",
        )
        .bind(file_id)
        .bind(grantee)
        .bind(permission_level)
        .bind(granted_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_file_permission(
        &self,
        file_id: Uuid,
        grantee: Uuid,
    ) -> Result<Option<FilePermission>, Box<DatabaseError>> {
        sqlx::query_as::<_, FilePermission>(
            "SELECT * FROM file_permissions WHERE file_id = $1 AND grantee = $2",
        )
        .bind(file_id)
        .bind(grantee)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn read_file_permissions(
        &self,
        file_id: Uuid,
    ) -> Result<Vec<FilePermission>, Box<DatabaseError>> {
        sqlx::query_as::<_, FilePermission>(
            "SELECT * FROM file_permissions WHERE file_id = $1 ORDER BY created_at ASC",
        )
        .bind(file_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))
    }

    async fn delete_file_permission(
        &mut self,
        file_id: Uuid,
        grantee: Uuid,
    ) -> Result<bool, Box<DatabaseError>> {
        let rows = sqlx::query(
            "DELETE FROM file_permissions WHERE file_id = $1 AND grantee = $2",
        )
        .bind(file_id)
        .bind(grantee)
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(DatabaseError::Sqlx(e)))?;
        Ok(rows.rows_affected() == 1)
    }
}
