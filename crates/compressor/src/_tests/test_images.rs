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
        .join("tests/assets/test_picture.jpg");
    let image = std::fs::read(&img_path)
        .unwrap_or_else(|_| panic!("Put a small JPG at {}", img_path.to_string_lossy()));
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
    let out =
        compress_image(&image, &params).expect("NoCompression should return the same");
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
