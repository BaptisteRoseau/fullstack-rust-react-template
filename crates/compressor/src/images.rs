use crate::error::CompressorError;
use crate::parameters::{
    ImageCompression, ImageConversion, ImageParameters, ImageResize,
};
use caesium::compress_in_memory;
use caesium::parameters::{
    ChromaSubsampling, GifParameters, JpegParameters, PngParameters, TiffDeflateLevel,
    TiffParameters, WebPParameters,
};
use caesium::{SupportedFileTypes, convert_in_memory, parameters::CSParameters};

pub fn compress_image(
    image: &[u8],
    parameters: &ImageParameters,
) -> Result<Vec<u8>, Box<CompressorError>> {
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
) -> Result<Vec<u8>, Box<CompressorError>> {
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
#[path = "_tests/test_images.rs"]
mod tests;
