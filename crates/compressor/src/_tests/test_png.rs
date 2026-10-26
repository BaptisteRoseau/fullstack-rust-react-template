use super::*;
use crate::images::compress_image;
use crate::parameters::{ImageCompression, ImageConversion, ImageParameters};

const NO_RESIZE: ImageResize = ImageResize {
    height: None,
    width: None,
};

fn load_jpg_image() -> Vec<u8> {
    let img_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/assets/test_picture.jpg");
    std::fs::read(&img_path)
        .unwrap_or_else(|_| panic!("Put a small JPG at {}", img_path.to_string_lossy()))
}

fn load_png_image() -> Vec<u8> {
    compress_to_png(&load_jpg_image(), &NO_RESIZE).expect("JPG to PNG should succeed")
}

#[test]
fn test_is_png() {
    assert!(is_png(&load_png_image()), "a PNG should be detected as PNG");
    assert!(!is_png(&load_jpg_image()), "a JPG should not be detected as PNG");
    assert!(!is_png(b"not an image"), "text should not be detected as PNG");
}

#[test]
fn test_scale_keeps_aspect_ratio() {
    let scaled = scale(800, 300, 600);
    assert_eq!(scaled, 400, "scaled: {scaled}");
}

#[test]
fn test_compress_to_png_converts_jpg() {
    let jpg = load_jpg_image();
    let png = compress_to_png(&jpg, &NO_RESIZE).expect("conversion should succeed");
    assert!(is_png(&png), "output should be a PNG, first bytes: {:?}", &png[..8]);
}

#[test]
fn test_compress_to_png_resizes_with_aspect_ratio() {
    let original = image::load_from_memory(&load_jpg_image()).expect("JPG should decode");
    let resize = ImageResize {
        height: Some(100),
        width: None,
    };

    let png = compress_to_png(&load_jpg_image(), &resize).expect("resize should succeed");
    let resized = image::load_from_memory(&png).expect("PNG should decode");

    let expected_width = scale(original.width(), 100, original.height());
    assert_eq!(resized.height(), 100, "height: {}", resized.height());
    assert_eq!(
        resized.width(),
        expected_width,
        "width: {}, expected: {}",
        resized.width(),
        expected_width
    );
}

#[test]
fn test_compress_to_png_does_not_grow_a_png() {
    let png = load_png_image();
    let optimized = compress_to_png(&png, &NO_RESIZE).expect("optimization should succeed");
    assert!(
        optimized.len() <= png.len(),
        "optimized: {}, original: {}",
        optimized.len(),
        png.len()
    );
}

#[test]
fn test_compress_image_lossy_on_png() {
    let png = load_png_image();
    let params = ImageParameters {
        compression: ImageCompression::Lossy,
        conversion: ImageConversion::NoConversion,
        resize: NO_RESIZE,
    };

    let out = compress_image(&png, &params).expect("compression should succeed");
    assert!(is_png(&out), "output should stay a PNG, first bytes: {:?}", &out[..8]);
}

#[test]
fn test_compress_image_converts_png_to_webp() {
    let png = load_png_image();
    let params = ImageParameters {
        compression: ImageCompression::Lossy,
        conversion: ImageConversion::Webp,
        resize: NO_RESIZE,
    };

    let out = compress_image(&png, &params).expect("conversion should succeed");
    let format = image::guess_format(&out);
    assert!(
        matches!(format, Ok(ImageFormat::WebP)),
        "output format: {format:?}"
    );
}

#[test]
fn test_compress_image_png_to_png_without_compression_returns_same() {
    let png = load_png_image();
    let params = ImageParameters {
        compression: ImageCompression::NoCompression,
        conversion: ImageConversion::Png,
        resize: NO_RESIZE,
    };

    let out = compress_image(&png, &params).expect("conversion should succeed");
    assert_eq!(out.len(), png.len(), "out: {}, png: {}", out.len(), png.len());
}
