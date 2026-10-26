use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};

use crate::error::CompressorError;
use crate::parameters::ImageResize;

const OXIPNG_PRESET: u8 = 3;

pub fn is_png(image: &[u8]) -> bool {
    matches!(image::guess_format(image), Ok(ImageFormat::Png))
}

/// Resizes the image if asked, encodes it as PNG and optimizes it losslessly.
pub fn compress_to_png(
    image: &[u8],
    resize: &ImageResize,
) -> Result<Vec<u8>, Box<CompressorError>> {
    let needs_resize = resize.height.is_some() || resize.width.is_some();
    if is_png(image) && !needs_resize {
        return optimize(image);
    }

    let decoded = image::load_from_memory(image)?;
    let resized = resize_image(decoded, resize);

    let mut encoded = Vec::new();
    resized.write_to(&mut Cursor::new(&mut encoded), ImageFormat::Png)?;
    optimize(&encoded)
}

fn optimize(png: &[u8]) -> Result<Vec<u8>, Box<CompressorError>> {
    let options = oxipng::Options::from_preset(OXIPNG_PRESET);
    Ok(oxipng::optimize_from_memory(png, &options)?)
}

fn resize_image(image: DynamicImage, resize: &ImageResize) -> DynamicImage {
    let (width, height) = match (resize.width, resize.height) {
        (None, None) => return image,
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, scale(image.height(), width, image.width())),
        (None, Some(height)) => (scale(image.width(), height, image.height()), height),
    };
    image.resize_exact(width, height, FilterType::Lanczos3)
}

/// Scales `side` by the `new_other / old_other` ratio, to keep the aspect ratio.
fn scale(side: u32, new_other: u32, old_other: u32) -> u32 {
    let scaled = side as f64 * new_other as f64 / old_other as f64;
    scaled.round() as u32
}

test_utils::tests_file!("_tests/test_png.rs");
