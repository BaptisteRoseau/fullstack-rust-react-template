# Compressor

Byte-level and image compression. Used by [storage](../storage) before a blob is written,
and to decompress it on read. [api](../api) also depends on it directly, to build
`CompressionParameters` from an upload request's query parameters.

This is not a service crate: it exposes plain functions, not a trait, because there is
only one implementation and no backend to swap.

## Public surface

- [`parameters`](src/parameters.rs) — `CompressionParameters`, the struct callers build to
  describe what to do. `CompressionParameters::compressed()` and `::compressed_lossy()`
  are the two ready-made presets; `with_image_conversion` / `with_image_resize` refine
  them further. `Default` performs no compression at all.
- [`compressor`](src/compressor.rs) — `handle_compression` / `handle_decompression`,
  dispatching on `parameters.compression` (currently gzip or none), plus the underlying
  `compress_bytes` / `decompress_bytes`.
- [`images`](src/images.rs) — `compress_image`, used when the blob is an image. Wraps the
  `libcaesium` crate to additionally resize and convert format (JPEG, PNG, TIFF, WebP).
- [`error::CompressorError`](src/error.rs) — wraps `caesium::error::CaesiumError` and
  `std::io::Error`.

## Directory

```txt
compressor/
├── src/
│   ├── compressor.rs   # generic byte (de)compression, gzip today
│   ├── images.rs       # image-specific compression, resize and format conversion
│   ├── parameters.rs   # CompressionParameters and its presets
│   └── error.rs
└── tests/
    └── assets/         # sample image used by the image compression tests
```
