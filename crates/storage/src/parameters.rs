#[derive(Copy, Clone, PartialEq)]
pub enum EncryptionType {
    NoEncryption,
    Encryption,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Compression {
    NoCompression,
    Gzip,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ImageCompression {
    NoCompression,
    Lossless,
    Lossy,
    Auto,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ImageConvertion {
    NoConvertion,
    Webp,
    Jpeg,
    Png,
    Tiff,
}

#[derive(Copy, Clone, PartialEq)]
pub struct ImageResize {
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Copy, Clone, PartialEq)]
pub struct ImageParameters {
    pub compression: ImageCompression,
    pub convertion: ImageConvertion,
    pub resize: ImageResize,
}

/// Struct representing overall compression parameters for storage.
///
/// Fields:
///     TODO: Documentation of the whole parameters.
#[derive(Copy, Clone)]
pub struct StorageParameters {
    encryption: EncryptionType,
    compression: Compression,
    image: ImageParameters,
}

impl Default for StorageParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageParameters {
    pub fn new() -> StorageParameters {
        initialize_parameters()
    }
}

fn initialize_parameters() -> StorageParameters {
    StorageParameters {
        encryption: EncryptionType::NoEncryption,
        compression: Compression::Gzip,
        image: ImageParameters {
            compression: ImageCompression::Auto,
            convertion: ImageConvertion::NoConvertion,
            resize: ImageResize {
                height: None,
                width: None,
            },
        },
    }
}
