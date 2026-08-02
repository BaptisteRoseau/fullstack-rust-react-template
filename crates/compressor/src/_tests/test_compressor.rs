use super::*;

#[test]
fn test_compress_decompress() {
    let s = "The quick brown fox jumps over the lazy dog.";
    let original = s.repeat(200).into_bytes();
    let compressed = compress_bytes(&original).expect("compression failed");
    assert!(
        compressed.len() < original.len(),
        "compressed should be smaller than original, got compressed {} and original {}",
        compressed.len(),
        original.len()
    );
    let decompressed = decompress_bytes(&compressed).expect("decompression failed");
    assert_eq!(
        decompressed, original,
        "decompressed data should equal original"
    );
}

#[test]
fn test_handle_compress_decompress_gzip() {
    let s = "The quick brown fox jumps over the lazy dog.";
    let original = s.repeat(200).into_bytes();

    let compression_parameters = CompressionParameters::Gzip;
    let compressed = handle_compression(&original, compression_parameters)
        .expect("compression failed");
    assert!(
        compressed.len() < original.len(),
        "compressed should be smaller than original, got compressed {} and original {}",
        compressed.len(),
        original.len()
    );

    let decompressed = handle_decompression(&compressed, compression_parameters)
        .expect("decompression failed");
    assert_eq!(
        decompressed, original,
        "decompressed data should equal original"
    );
}

#[test]
fn test_handle_compress_decompress_no_compression() {
    let s = "The quick brown fox jumps over the lazy dog.";
    let original = s.repeat(200).into_bytes();

    let compression_parameters = CompressionParameters::NoCompression;
    let compressed = handle_compression(&original, compression_parameters)
        .expect("compression failed");
    assert_eq!(compressed.len(), original.len());

    let decompressed = handle_decompression(&compressed, compression_parameters)
        .expect("decompression failed");
    assert_eq!(decompressed, original);
}
