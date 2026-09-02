//! Stored files: the compress-then-encrypt pipeline and the operations around
//! it.
//!
//! Upload order matters. Compression runs **first**, because ciphertext is
//! incompressible; encryption runs second, so the object store only ever holds
//! opaque bytes under a key that names neither the file nor its type.
//!
//! ```txt
//! upload:   bytes -> image compression -> gzip -> AES-256-GCM -> storage
//! download: storage -> AES-256-GCM -> gunzip -> bytes
//! ```
//!
//! [`storage::Storage`] applies its own compression, so every call here passes
//! [`CompressionParameters::default()`], which it treats as a byte-for-byte
//! passthrough. Handing it anything else would compress ciphertext.

use std::path::PathBuf;

use compressor::compressor::{handle_compression, handle_decompression};
use compressor::images::compress_image;
use compressor::parameters::{
    Compression, CompressionParameters, ImageCompression, ImageConversion,
    ImageParameters, ImageResize,
};
use database::Database;
use database::models::{File, NewFile};
use rbac::PermissionLevel;
use storage::Storage;
use uuid::Uuid;

use crate::access::{self, ResourceRef};
use crate::directory::{check_move_destination, validate_name};
use crate::encryption::{self, KEY_LENGTH};
use crate::error::CoreError;

/// Longest side, in pixels, of the thumbnail generated for an image.
pub const THUMBNAIL_WIDTH: u32 = 320;

/// The bytes and metadata of one incoming upload.
#[derive(Debug)]
pub struct Upload {
    pub name: String,
    pub mime_type: String,
    pub content: Vec<u8>,
}

/// Runs the full pipeline for one upload and records the row.
///
/// Needs [`PermissionLevel::Editor`] on `parent_id`; a `None` parent is the
/// caller's own root, which needs no check.
pub async fn upload_file(
    db: &mut dyn Database,
    blobs: &dyn Storage,
    master_key: &[u8; KEY_LENGTH],
    user_id: Uuid,
    parent_id: Option<Uuid>,
    upload: Upload,
) -> Result<File, CoreError> {
    let name = validate_name(upload.name)?;
    if let Some(parent_id) = parent_id {
        access::require(
            db,
            user_id,
            ResourceRef::Directory(parent_id),
            PermissionLevel::Editor,
        )
        .await?;
    }

    let original_size = upload.content.len();
    let mime_type = upload.mime_type;

    let compressed = compress_for_storage(&upload.content, &mime_type)?;
    let data_key = encryption::generate_data_key();
    let content = encryption::seal(&data_key, &compressed.bytes)?;
    let wrapped_key = encryption::wrap_data_key(master_key, &data_key)?;

    let id = Uuid::now_v7();
    let storage_key = content_key(id);
    blobs
        .save(
            &PathBuf::from(&storage_key),
            &content.ciphertext,
            &CompressionParameters::default(),
        )
        .await?;

    let thumbnail =
        store_thumbnail(blobs, &data_key, id, &upload.content, &mime_type).await?;

    let row = db
        .create_file(NewFile {
            id,
            owner: user_id,
            parent_id,
            name,
            storage_key: storage_key.clone(),
            mime_type,
            size_bytes: original_size as i64,
            stored_size_bytes: content.ciphertext.len() as i64,
            is_compressed: compressed.gzipped,
            encrypted_dek: wrapped_key.ciphertext,
            dek_nonce: wrapped_key.nonce.to_vec(),
            content_nonce: content.nonce.to_vec(),
            thumbnail_storage_key: thumbnail.as_ref().map(|_| thumbnail_key(id)),
            thumbnail_nonce: thumbnail.map(|nonce| nonce.to_vec()),
        })
        .await;

    match row {
        Ok(row) => Ok(row),
        Err(error) => {
            // The blobs are already written and nothing points at them any
            // more, so drop them rather than leak an unreachable object.
            let _ = blobs.delete(&PathBuf::from(&storage_key)).await;
            let _ = blobs.delete(&PathBuf::from(thumbnail_key(id))).await;
            Err(CoreError::DatabaseError(error))
        }
    }
}

/// Reads one file's metadata. Needs [`PermissionLevel::Viewer`].
pub async fn read_file(
    db: &dyn Database,
    user_id: Uuid,
    id: Uuid,
) -> Result<File, CoreError> {
    access::require(db, user_id, ResourceRef::File(id), PermissionLevel::Viewer).await?;
    Ok(db.read_file(id).await?)
}

/// Returns the row and the original bytes, decrypted and decompressed. Needs
/// [`PermissionLevel::Viewer`].
pub async fn download_file(
    db: &dyn Database,
    blobs: &dyn Storage,
    master_key: &[u8; KEY_LENGTH],
    user_id: Uuid,
    id: Uuid,
) -> Result<(File, Vec<u8>), CoreError> {
    let file = read_file(db, user_id, id).await?;

    let ciphertext = blobs.load(&PathBuf::from(&file.storage_key)).await?;
    let data_key =
        encryption::unwrap_data_key(master_key, &file.encrypted_dek, &file.dek_nonce)?;
    let compressed = encryption::open(&data_key, &file.content_nonce, &ciphertext)?;

    let content = if file.is_compressed {
        handle_decompression(&compressed, Compression::Gzip)?
    } else {
        compressed
    };

    Ok((file, content))
}

/// Returns the decrypted WebP thumbnail. Needs [`PermissionLevel::Viewer`].
/// Answers [`CoreError::NotFound`] for a file that has none.
pub async fn download_thumbnail(
    db: &dyn Database,
    blobs: &dyn Storage,
    master_key: &[u8; KEY_LENGTH],
    user_id: Uuid,
    id: Uuid,
) -> Result<Vec<u8>, CoreError> {
    let file = read_file(db, user_id, id).await?;

    let (Some(key), Some(nonce)) = (&file.thumbnail_storage_key, &file.thumbnail_nonce)
    else {
        return Err(CoreError::NotFound(format!("thumbnail of {id}")));
    };

    let ciphertext = blobs.load(&PathBuf::from(key)).await?;
    let data_key =
        encryption::unwrap_data_key(master_key, &file.encrypted_dek, &file.dek_nonce)?;
    encryption::open(&data_key, nonce, &ciphertext)
}

/// Renames and/or moves a file. Needs [`PermissionLevel::Editor`] on it, and on
/// the destination directory when it moves.
pub async fn update_file(
    db: &mut dyn Database,
    user_id: Uuid,
    id: Uuid,
    name: Option<String>,
    parent_id: Option<Option<Uuid>>,
) -> Result<File, CoreError> {
    access::require(db, user_id, ResourceRef::File(id), PermissionLevel::Editor).await?;

    let name = name.map(validate_name).transpose()?;

    if let Some(destination) = parent_id {
        check_move_destination(db, user_id, destination, None).await?;
        if destination.is_none() && db.read_file(id).await?.owner != user_id {
            return Err(CoreError::InvalidRequest(
                "only the owner may move a file to the root".to_string(),
            ));
        }
    }

    Ok(db.update_file(id, name, parent_id).await?)
}

/// Deletes the row and both stored blobs. Needs [`PermissionLevel::Manager`].
pub async fn delete_file(
    db: &mut dyn Database,
    blobs: &dyn Storage,
    user_id: Uuid,
    id: Uuid,
) -> Result<(), CoreError> {
    access::require(db, user_id, ResourceRef::File(id), PermissionLevel::Manager).await?;

    let file = db.read_file(id).await?;
    if !db.delete_file(id).await? {
        return Err(CoreError::NotFound(id.to_string()));
    }

    blobs.delete(&PathBuf::from(&file.storage_key)).await?;
    if let Some(key) = &file.thumbnail_storage_key {
        blobs.delete(&PathBuf::from(key)).await?;
    }
    Ok(())
}

/// The result of the compression half of the pipeline.
struct Compressed {
    bytes: Vec<u8>,
    /// Whether gzip was applied, which the download path has to reverse.
    gzipped: bool,
}

/// Shrinks the payload as much as is worth doing before it becomes
/// incompressible ciphertext: a lossy pass over images, then gzip for
/// everything that is not already a compressed container.
fn compress_for_storage(
    content: &[u8],
    mime_type: &str,
) -> Result<Compressed, CoreError> {
    let shrunk = if is_image(mime_type) {
        compress_image(content, &lossy_image_parameters())?
    } else {
        content.to_vec()
    };

    if is_already_compressed(mime_type) {
        return Ok(Compressed {
            bytes: shrunk,
            gzipped: false,
        });
    }

    Ok(Compressed {
        bytes: handle_compression(&shrunk, Compression::Gzip)?,
        gzipped: true,
    })
}

/// Writes the encrypted thumbnail of an image and answers the nonce it was
/// sealed with, or `None` for anything that is not an image.
///
/// The thumbnail reuses the file's data encryption key — it is the same secret,
/// shared with the same people — but never its nonce: reusing a nonce under one
/// GCM key breaks the cipher.
async fn store_thumbnail(
    blobs: &dyn Storage,
    data_key: &[u8; KEY_LENGTH],
    id: Uuid,
    content: &[u8],
    mime_type: &str,
) -> Result<Option<[u8; encryption::NONCE_LENGTH]>, CoreError> {
    if !is_image(mime_type) {
        return Ok(None);
    }

    let thumbnail = compress_image(content, &thumbnail_parameters())?;
    let sealed = encryption::seal(data_key, &thumbnail)?;
    blobs
        .save(
            &PathBuf::from(thumbnail_key(id)),
            &sealed.ciphertext,
            &CompressionParameters::default(),
        )
        .await?;

    Ok(Some(sealed.nonce))
}

/// Neither key carries the file name nor its type: the object store is not
/// trusted with metadata, only with opaque bytes.
fn content_key(id: Uuid) -> String {
    format!("files/{id}/content")
}

fn thumbnail_key(id: Uuid) -> String {
    format!("files/{id}/thumbnail")
}

fn lossy_image_parameters() -> ImageParameters {
    ImageParameters {
        compression: ImageCompression::Lossy,
        conversion: ImageConversion::NoConversion,
        resize: ImageResize {
            height: None,
            width: None,
        },
    }
}

fn thumbnail_parameters() -> ImageParameters {
    ImageParameters {
        compression: ImageCompression::Lossy,
        conversion: ImageConversion::Webp,
        resize: ImageResize {
            height: None,
            width: Some(THUMBNAIL_WIDTH),
        },
    }
}

fn is_image(mime_type: &str) -> bool {
    mime_type.starts_with("image/")
}

/// Formats whose bytes are already entropy-dense. Gzipping them costs CPU on
/// both ends and gives back nothing, so the list stays to the containers that
/// actually turn up in an upload.
fn is_already_compressed(mime_type: &str) -> bool {
    const COMPRESSED_TYPES: [&str; 6] = [
        "image/jpeg",
        "image/png",
        "image/webp",
        "image/gif",
        "application/zip",
        "application/gzip",
    ];

    COMPRESSED_TYPES.contains(&mime_type)
        || mime_type.starts_with("video/")
        || mime_type.starts_with("audio/")
}

test_utils::tests_file!("_tests/test_file.rs");
