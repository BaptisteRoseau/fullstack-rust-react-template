use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{self, Cursor, Read, Write};

use crate::parameters::Compression as CompressionParameters;

pub fn handle_compression(
    input: &[u8],
    compression_parameters: CompressionParameters,
) -> io::Result<Vec<u8>> {
    match compression_parameters {
        CompressionParameters::Gzip => compress_bytes(input),
        CompressionParameters::NoCompression => Ok(input.into()),
    }
}

pub fn handle_decompression(
    input: &[u8],
    compression_parameters: CompressionParameters,
) -> io::Result<Vec<u8>> {
    match compression_parameters {
        CompressionParameters::Gzip => decompress_bytes(input),
        CompressionParameters::NoCompression => Ok(input.into()),
    }
}

pub fn compress_bytes(input: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub fn decompress_bytes(input: &[u8]) -> io::Result<Vec<u8>> {
    let cursor = Cursor::new(input);
    let mut decoder = GzDecoder::new(cursor);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

#[cfg(test)]
#[path = "_tests/test_compressor.rs"]
mod tests;
