// TODO: Auto-select parameters and/or make prebuilt ones.
//
// This file selects how to handle a file's compression, encryption
// and optimization based of its filemime.
//
// For saving, it will:
//     1. Optimize the file based on its type (ex. PNG)
//     2. Compress the file based on the given parameters
//     3. Encrypt the file based on the given parameters
//
// For loading, it will:
//     1. Decrypt the file
//     2. Decompress the file
//
// If the "auto" options are selected, it will select special options
// like lossy image compression or resizing based of the size of
// the file being stored.

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
pub enum ImageConversion {
    NoConversion,
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
    pub conversion: ImageConversion,
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

    pub fn to_webp(&mut self) -> &mut Self {
        self.image.conversion = ImageConversion::Webp;
        self
    }

    pub fn no_compression(&mut self) -> &mut Self {
        self.compression = Compression::NoCompression;
        self
    }
}

fn initialize_parameters() -> StorageParameters {
    StorageParameters {
        encryption: EncryptionType::NoEncryption,
        compression: Compression::Gzip,
        image: ImageParameters {
            compression: ImageCompression::Auto,
            conversion: ImageConversion::NoConversion,
            resize: ImageResize {
                height: None,
                width: None,
            },
        },
    }
}
