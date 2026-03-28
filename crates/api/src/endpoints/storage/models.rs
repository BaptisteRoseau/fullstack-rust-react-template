use serde::{Deserialize, Serialize};
use storage::parameters::{
    Compression, ImageCompression, ImageConversion, StorageParameters,
};
use utoipa::{IntoParams, ToResponse, ToSchema};

/// Query parameters for file upload, controlling compression and image processing.
/// All fields are optional; defaults to gzip compression with lossy image compression.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub(crate) struct PostUploadParams {
    /// Enable gzip compression. Defaults to true.
    pub compression: Option<bool>,
    /// Image compression mode: "none", "lossless", "lossy", "auto". Defaults to "lossy".
    pub image_compression: Option<String>,
    /// Image conversion format: "none", "webp", "jpeg", "png", "tiff". Defaults to no conversion.
    pub image_conversion: Option<String>,
    /// Desired image height for resizing.
    pub image_height: Option<u32>,
    /// Desired image width for resizing.
    pub image_width: Option<u32>,
}

impl PostUploadParams {
    pub fn into_storage_parameters(self) -> StorageParameters {
        let mut params = StorageParameters::compressed_lossy();

        if let Some(false) = self.compression {
            params.compression = Compression::NoCompression;
        }

        if let Some(ref ic) = self.image_compression {
            let compression = match ic.as_str() {
                "none" => ImageCompression::NoCompression,
                "lossless" => ImageCompression::Lossless,
                "auto" => ImageCompression::Auto,
                _ => ImageCompression::Lossy,
            };
            params.with_image_compression(compression);
        }

        if let Some(ref conv) = self.image_conversion {
            let conversion = match conv.as_str() {
                "webp" => ImageConversion::Webp,
                "jpeg" => ImageConversion::Jpeg,
                "png" => ImageConversion::Png,
                "tiff" => ImageConversion::Tiff,
                _ => ImageConversion::NoConversion,
            };
            params.with_image_conversion(conversion);
        }

        if self.image_height.is_some() || self.image_width.is_some() {
            params.with_image_resize(self.image_height, self.image_width);
        }

        params
    }
}

/// Response returned after a successful file upload.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub(crate) struct PostUploadResponse {
    pub file: String,
}
