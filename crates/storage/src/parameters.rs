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

impl Default for ImageParameters {
    fn default() -> Self {
        Self {
            compression: ImageCompression::NoCompression,
            conversion: ImageConversion::NoConversion,
            resize: ImageResize {
                height: None,
                width: None,
            },
        }
    }
}

#[derive(Copy, Clone)]
pub struct StorageParameters {
    pub compression: Compression,
    pub image: ImageParameters,
}

impl Default for StorageParameters {
    /// Default parameters, does not alter the file.
    fn default() -> Self {
        StorageParameters {
            compression: Compression::NoCompression,
            image: ImageParameters::default(),
        }
    }
}

impl StorageParameters {
    /// Compress the file. Images are compressed using a lossless compression algorithm.
    pub fn compressed() -> Self {
        *Self::default()
            .with_compression()
            .with_image_compression(ImageCompression::Lossless)
    }

    /// Compress the file. Images are compressed using a lossy compression algorithm.
    pub fn compressed_lossy() -> Self {
        *Self::default()
            .with_compression()
            .with_image_compression(ImageCompression::Lossy)
    }

    pub fn with_compression(&mut self) -> &mut Self {
        self.compression = Compression::Gzip;
        self
    }

    pub fn with_image_compression(&mut self, compression: ImageCompression) -> &mut Self {
        self.image.compression = compression;
        self
    }

    pub fn with_image_conversion(&mut self, conversion: ImageConversion) -> &mut Self {
        self.image.conversion = conversion;
        self
    }

    /// Allows to resize the image to the desired size.
    pub fn with_image_resize(
        &mut self,
        height: Option<u32>,
        width: Option<u32>,
    ) -> &mut Self {
        self.image.resize = ImageResize { height, width };
        self
    }
}
