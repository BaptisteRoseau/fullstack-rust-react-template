//! Envelope encryption for stored files.
//!
//! Every file carries its own random 256-bit **data encryption key** (DEK). The
//! content is sealed with AES-256-GCM under that key; the key itself is sealed
//! under the server-wide master key from
//! [`config::StorageConfig::encryption_key`] and stored next to the row. Only
//! the wrapped key ever touches the database, and only ciphertext ever reaches
//! the object store.
//!
//! Rotating the master key therefore means re-wrapping the stored keys, not
//! re-encrypting the files.

use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, Key, KeyInit, Nonce};

use crate::error::CoreError;

/// Length of a data encryption key and of the master key, set by AES-256.
pub const KEY_LENGTH: usize = 32;

/// Length of a GCM nonce. A fresh one is drawn for every sealed payload; the
/// same nonce is never reused under the same key.
pub const NONCE_LENGTH: usize = 12;

/// A sealed payload and the nonce needed to open it.
#[derive(Debug, Clone)]
pub struct Sealed {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; NONCE_LENGTH],
}

/// Draws a new data encryption key from the system random source.
pub fn generate_data_key() -> [u8; KEY_LENGTH] {
    rand::random()
}

/// Draws a nonce that has never been used before, with overwhelming
/// probability, for a single sealing operation.
pub fn generate_nonce() -> [u8; NONCE_LENGTH] {
    rand::random()
}

/// Seals `plaintext` under `key` with a freshly drawn nonce.
pub fn seal(key: &[u8; KEY_LENGTH], plaintext: &[u8]) -> Result<Sealed, CoreError> {
    let nonce = generate_nonce();
    Ok(Sealed {
        ciphertext: seal_with_nonce(key, &nonce, plaintext)?,
        nonce,
    })
}

/// Seals `plaintext` under `key` with a caller-chosen nonce. Callers sealing
/// two payloads under one key — a file and its thumbnail — use this to stay
/// explicit that each payload gets its own nonce.
pub fn seal_with_nonce(
    key: &[u8; KEY_LENGTH],
    nonce: &[u8; NONCE_LENGTH],
    plaintext: &[u8],
) -> Result<Vec<u8>, CoreError> {
    cipher(key)?
        .encrypt(&as_nonce(nonce)?, plaintext)
        .map_err(|_| CoreError::Encryption)
}

/// Opens `ciphertext` sealed under `key` and `nonce`.
///
/// Fails the same way on a wrong key, a wrong nonce and tampered bytes: GCM
/// authenticates before it decrypts, and the three are not distinguishable.
pub fn open(
    key: &[u8; KEY_LENGTH],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CoreError> {
    cipher(key)?
        .decrypt(&as_nonce(nonce)?, ciphertext)
        .map_err(|_| CoreError::Encryption)
}

/// Seals a data encryption key under the master key, producing the
/// `encrypted_dek` / `dek_nonce` pair stored on the file row.
pub fn wrap_data_key(
    master_key: &[u8; KEY_LENGTH],
    data_key: &[u8; KEY_LENGTH],
) -> Result<Sealed, CoreError> {
    seal(master_key, data_key)
}

/// Recovers a data encryption key from its stored `encrypted_dek` /
/// `dek_nonce` pair.
pub fn unwrap_data_key(
    master_key: &[u8; KEY_LENGTH],
    encrypted_data_key: &[u8],
    nonce: &[u8],
) -> Result<[u8; KEY_LENGTH], CoreError> {
    open(master_key, nonce, encrypted_data_key)?
        .try_into()
        .map_err(|_| CoreError::Encryption)
}

fn cipher(key: &[u8; KEY_LENGTH]) -> Result<Aes256Gcm, CoreError> {
    let key = Key::<Aes256Gcm>::try_from(&key[..]).map_err(|_| CoreError::Encryption)?;
    Ok(Aes256Gcm::new(&key))
}

fn as_nonce(nonce: &[u8]) -> Result<Nonce<Aes256Gcm>, CoreError> {
    Nonce::<Aes256Gcm>::try_from(nonce).map_err(|_| CoreError::Encryption)
}

test_utils::tests_file!("_tests/test_encryption.rs");
