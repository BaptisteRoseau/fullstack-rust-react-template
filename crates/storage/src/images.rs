use crate::error::StorageError;
use caesium::compress_in_memory;
use caesium::parameters::{
    ChromaSubsampling, GifParameters, JpegParameters, PngParameters, TiffDeflateLevel,
    TiffParameters, WebPParameters,
};
use caesium::{SupportedFileTypes, convert_in_memory, parameters::CSParameters};

pub(crate) enum CompressionType {
    Lossy,
    Lossless,
}

pub(crate) fn compress_image(
    image: &[u8],
    compression_type: CompressionType,
) -> Result<Vec<u8>, StorageError> {
    let parameters = select_parameters(compression_type);
    Ok(compress_in_memory(image.into(), &parameters)?)
}

pub(crate) fn convert_image_to(
    image: &[u8],
    format: SupportedFileTypes,
    compression_type: CompressionType,
) -> Result<Vec<u8>, StorageError> {
    let parameters = select_parameters(compression_type);
    Ok(convert_in_memory(image.into(), &parameters, format)?)
}

pub(crate) fn compress_and_resize_image(
    image: &[u8],
    height: Option<u32>,
    width: Option<u32>,
    compression_type: CompressionType,
) -> Result<Vec<u8>, StorageError> {
    let mut parameters = select_parameters(compression_type);
    if let Some(h) = height {
        parameters.height = h;
    }
    if let Some(w) = width {
        parameters.width = w;
    }
    Ok(compress_in_memory(image.into(), &parameters)?)
}

fn select_parameters(compression_type: CompressionType) -> CSParameters {
    match compression_type {
        CompressionType::Lossless => parameters_lossless(),
        CompressionType::Lossy => parameters_lossy(),
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
        keep_metadata: false,
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
        keep_metadata: false,
        width: 0,
        height: 0,
    }
}
