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
) -> Result<Vec<u8>, StorageError> {
    if parameters.compression == ImageCompression::NoCompression {
        return Ok(image.into());
    }
    let mut compression_parameters = select_compression(&parameters.compression);
    update_dimensions(&parameters.resize, &mut compression_parameters);

    let format = match parameters.conversion {
        ImageConversion::Jpeg => SupportedFileTypes::Jpeg,
        ImageConversion::Png => SupportedFileTypes::Png,
        ImageConversion::Tiff => SupportedFileTypes::Tiff,
        ImageConversion::Webp => SupportedFileTypes::WebP,
        ImageConversion::NoConversion => {
            return Ok(compress_in_memory(image.into(), &compression_parameters)?);
        }
    };

    Ok(convert_in_memory(
        image.into(),
        &compression_parameters,
        format,
    )?)
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
    fn test_no_compression_returns_same() {
        let input: Vec<u8> = b"this is not an image".to_vec();
        let params = ImageParameters {
            compression: ImageCompression::NoCompression,
            conversion: ImageConversion::NoConversion,
            resize: ImageResize {
                height: None,
                width: None,
            },
        };

        let out =
            compress_image(&input, &params).expect("NoCompression should return Ok");
        assert_eq!(out, input);
    }

    #[test]
    fn test_select_compression() {
        let lossy = select_compression(&ImageCompression::Lossy);
        let lossless = select_compression(&ImageCompression::Lossless);

        assert!(lossy.jpeg.quality < 100);
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

    #[test]
    #[ignore]
    fn test_compress_in_memory_with_png_fixture() {
        todo!("This test requires a fixture that does not exists yet.");
        let img_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/1x1.png");
        let image = std::fs::read(&img_path)
            .expect("Put a small PNG at crates/storage/tests/fixtures/1x1.png");

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
        assert!(out.len() < image.len());
    }

    #[test]
    #[ignore]
    fn test_convert_image_format() {
        todo!("This test requires a fixture that does not exists yet.");
    }
}
