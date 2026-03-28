use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{self, Cursor, Read, Write};

use crate::parameters::Compression as CompressionParameters;

pub(crate) fn handle_compression(
    input: &[u8],
    compression_parameters: CompressionParameters,
) -> io::Result<Vec<u8>> {
    match compression_parameters {
        CompressionParameters::Gzip => compress_bytes(input),
        CompressionParameters::NoCompression => Ok(input.into()),
    }
}

pub(crate) fn handle_decompression(
    input: &[u8],
    compression_parameters: CompressionParameters,
) -> io::Result<Vec<u8>> {
    match compression_parameters {
        CompressionParameters::Gzip => decompress_bytes(input),
        CompressionParameters::NoCompression => Ok(input.into()),
    }
}

fn compress_bytes(input: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

fn decompress_bytes(input: &[u8]) -> io::Result<Vec<u8>> {
    let cursor = Cursor::new(input);
    let mut decoder = GzDecoder::new(cursor);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

#[cfg(test)]
mod test {
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
}
