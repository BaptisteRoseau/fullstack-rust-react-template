use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{self, Cursor, Read, Write};

pub(crate) fn compress_bytes(input: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub(crate) fn decompress_bytes(input: &[u8]) -> io::Result<Vec<u8>> {
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
}
