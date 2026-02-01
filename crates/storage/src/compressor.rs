use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{self, Cursor, Read, Write};

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

fn compress_stream<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    io::copy(&mut reader, &mut encoder)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}

fn decompress_stream<R: Read>(reader: R) -> io::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(reader);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
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
            "compressed should be smaller than original"
        );
        let decompressed = decompress_bytes(&compressed).expect("decompression failed");
        assert_eq!(
            decompressed, original,
            "decompressed data should equal original"
        );
    }

    #[test]
    fn test_compress_decompress_stream() {
        let s = "The quick brown fox jumps over the lazy dog.";
        let original = s.repeat(200).into_bytes();

        let compressed = super::compress_stream(std::io::Cursor::new(&original))
            .expect("compression stream failed");

        assert!(compressed.len() < original.len(), "compressed should be smaller than original");

        let decompressed = super::decompress_stream(std::io::Cursor::new(&compressed))
            .expect("decompression stream failed");

        assert_eq!(decompressed, original, "decompressed stream should equal original");
    }
}
