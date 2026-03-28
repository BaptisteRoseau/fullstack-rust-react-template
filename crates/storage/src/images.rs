use crate::error::StorageError;
use crate::parameters::{
    ImageCompression, ImageConversion, ImageParameters, ImageResize,
};
use caesium::compress_in_memory;
use caesium::parameters::{
    ChromaSubsampling, GifParameters, JpegParameters, PngParameters, TiffDeflateLevel,
    TiffParameters, WebPParameters,
};
use caesium::{SupportedFileTypes, convert_in_memory, parameters::CSParameters};

pub(crate) fn compress_image(
    image: &[u8],
    parameters: &ImageParameters,
) -> Result<Vec<u8>, Box<StorageError>> {
    let mut compression_parameters = select_compression(&parameters.compression);
    update_dimensions(&parameters.resize, &mut compression_parameters);

    let format = match parameters.conversion {
        ImageConversion::Jpeg => SupportedFileTypes::Jpeg,
        ImageConversion::Png => SupportedFileTypes::Png,
        ImageConversion::Tiff => SupportedFileTypes::Tiff,
        ImageConversion::Webp => SupportedFileTypes::WebP,
        ImageConversion::NoConversion => {
            return handle_compression_only(image, parameters, &compression_parameters);
        }
    };

    match convert_in_memory(image.into(), &compression_parameters, format) {
        Ok(compressed_image) => Ok(compressed_image),
        Err(e) => {
            if e.code == 10407 {
                // "Cannot convert to the same format" => Handle compression only
                handle_compression_only(image, parameters, &compression_parameters)
            } else {
                Err(e.into())
            }
        }
    }
}

fn select_compression(compression_type: &ImageCompression) -> CSParameters {
    match compression_type {
        ImageCompression::Lossy => parameters_lossy(),
        _ => parameters_lossless(),
    }
}

fn update_dimensions(resize: &ImageResize, compression_parameters: &mut CSParameters) {
    if let Some(h) = resize.height {
        compression_parameters.height = h;
    }
    if let Some(w) = resize.width {
        compression_parameters.width = w;
    }
}

fn handle_compression_only(
    image: &[u8],
    parameters: &ImageParameters,
    compression_parameters: &CSParameters,
) -> Result<Vec<u8>, Box<StorageError>> {
    if matches!(parameters.compression, ImageCompression::NoCompression)
        && parameters.resize.height.is_none()
        && parameters.resize.width.is_none()
    {
        Ok(image.into())
    } else {
        Ok(compress_in_memory(image.into(), compression_parameters)?)
    }
}

fn parameters_lossless() -> CSParameters {
    let jpeg = JpegParameters {
        quality: 100,
        chroma_subsampling: ChromaSubsampling::Auto,
        progressive: true,
        optimize: true,
        preserve_icc: true,
    };
    let png = PngParameters {
        quality: 100,
        force_zopfli: false,
        optimization_level: 3,
        optimize: true,
    };
    let gif = GifParameters { quality: 100 };
    let webp = WebPParameters {
        quality: 100,
        lossless: true,
    };
    let tiff = TiffParameters {
        algorithm: caesium::parameters::TiffCompression::Deflate,
        deflate_level: TiffDeflateLevel::Balanced,
    };

    CSParameters {
        jpeg,
        png,
        gif,
        webp,
        tiff,
        keep_metadata: true,
        width: 0,
        height: 0,
    }
}

fn parameters_lossy() -> CSParameters {
    let jpeg = JpegParameters {
        quality: 80,
        chroma_subsampling: ChromaSubsampling::Auto,
        progressive: true,
        optimize: false,
        preserve_icc: true,
    };
    let png = PngParameters {
        quality: 80,
        force_zopfli: false,
        optimization_level: 3,
        optimize: false,
    };
    let gif = GifParameters { quality: 80 };
    let webp = WebPParameters {
        quality: 80,
        lossless: false,
    };
    let tiff = TiffParameters {
        algorithm: caesium::parameters::TiffCompression::Deflate,
        deflate_level: TiffDeflateLevel::Balanced,
    };

    CSParameters {
        jpeg,
        png,
        gif,
        webp,
        tiff,
        keep_metadata: true,
        width: 0,
        height: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::{
        ImageCompression, ImageConversion, ImageParameters, ImageResize,
    };

    #[test]
    fn test_select_compression() {
        let lossy = select_compression(&ImageCompression::Lossy);
        let lossless = select_compression(&ImageCompression::Lossless);

        assert!(lossy.jpeg.quality < 100, "{}", lossy.jpeg.quality);
        assert_eq!(lossless.jpeg.quality, 100);
    }

    #[test]
    fn test_update_dimensions() {
        let mut params = parameters_lossless();
        let resize = ImageResize {
            height: Some(42),
            width: Some(24),
        };
        update_dimensions(&resize, &mut params);
        assert_eq!(params.height, 42);
        assert_eq!(params.width, 24);
    }

    fn load_jpg_image() -> Vec<u8> {
        let img_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/testing/assets/test_picture.jpg");
        let image = std::fs::read(&img_path).unwrap_or_else(|_| {
            panic!("Put a small JPG at {}", img_path.to_string_lossy())
        });
        assert!(!image.is_empty());
        image
    }

    #[test]
    fn test_input_is_not_an_image_without_action_ok() {
        let image: Vec<u8> = b"this is not an image".to_vec();
        let params = ImageParameters::default();

        let compressed = compress_image(&image, &params);
        assert!(compressed.is_ok());
        assert_eq!(compressed.unwrap(), image);
    }

    #[test]
    fn test_input_is_not_an_image_with_action_err() {
        let image: Vec<u8> = b"this is not an image".to_vec();
        let params = ImageParameters {
            compression: ImageCompression::Lossy,
            conversion: ImageConversion::NoConversion,
            resize: ImageResize {
                height: None,
                width: None,
            },
        };

        let compressed = compress_image(&image, &params);
        assert!(compressed.is_err());
    }

    #[test]
    fn test_no_compression_returns_same() {
        let image = load_jpg_image();
        let params = ImageParameters::default();
        let out = compress_image(&image, &params)
            .expect("NoCompression should return the same");
        assert_eq!(image, out);
    }

    #[test]
    fn test_compress_in_memory_with_png_fixture() {
        let image = load_jpg_image();
        let params = ImageParameters {
            compression: ImageCompression::Lossy,
            conversion: ImageConversion::NoConversion,
            resize: ImageResize {
                height: None,
                width: None,
            },
        };

        let out = compress_image(&image, &params).expect("compression should succeed");
        assert!(!out.is_empty());
        assert!(
            out.len() < image.len(),
            "out: {}, image: {}",
            out.len(),
            image.len()
        );
    }

    #[test]
    fn test_convert_image_different_format() {
        let image = load_jpg_image();
        let params = ImageParameters {
            compression: ImageCompression::NoCompression,
            conversion: ImageConversion::Png,
            resize: ImageResize {
                height: None,
                width: None,
            },
        };

        let out = compress_image(&image, &params).expect("conversion should succeed");
        assert!(!out.is_empty());
        assert!(
            out.len() != image.len(),
            "out: {}, image: {}",
            out.len(),
            image.len()
        );

        //TODO: Find a way to test the image format has been changed
    }

    #[test]
    fn test_convert_image_same_format() {
        let image = load_jpg_image();
        let params = ImageParameters {
            compression: ImageCompression::NoCompression,
            conversion: ImageConversion::Jpeg,
            resize: ImageResize {
                height: None,
                width: None,
            },
        };

        let out = compress_image(&image, &params).expect("conversion should succeed");
        assert!(!out.is_empty());
        assert!(
            out.len() == image.len(),
            "out: {}, image: {}",
            out.len(),
            image.len()
        );
    }

    #[test]
    fn test_resize_image() {
        let image = load_jpg_image();
        let params = ImageParameters {
            compression: ImageCompression::NoCompression,
            conversion: ImageConversion::NoConversion,
            resize: ImageResize {
                height: Some(200),
                width: None,
            },
        };

        let out = compress_image(&image, &params).expect("compression should succeed");
        assert!(!out.is_empty());
        assert!(
            out.len() < image.len(),
            "out: {}, image: {}",
            out.len(),
            image.len()
        );
        //TODO: Find a way to test the image format has been changed
    }
}
