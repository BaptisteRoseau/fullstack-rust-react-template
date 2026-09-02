use super::*;

/// Repetitive enough that gzip has something to work with, which is what makes
/// the "did it actually compress" assertions meaningful.
fn text_payload() -> Vec<u8> {
    "the quick brown fox jumps over the lazy dog\n"
        .repeat(200)
        .into_bytes()
}

#[test]
fn image_types_are_recognised() {
    for mime_type in ["image/png", "image/jpeg", "image/webp", "image/avif"] {
        assert!(
            is_image(mime_type),
            "{mime_type} must be treated as an image, got false"
        );
    }
    for mime_type in ["text/plain", "application/pdf", "video/mp4"] {
        assert!(
            !is_image(mime_type),
            "{mime_type} must not be treated as an image, got true"
        );
    }
}

#[test]
fn already_dense_formats_skip_gzip() {
    for mime_type in [
        "image/jpeg",
        "image/png",
        "image/webp",
        "application/zip",
        "application/gzip",
        "video/mp4",
        "audio/mpeg",
    ] {
        assert!(
            is_already_compressed(mime_type),
            "{mime_type} must skip gzip, got false"
        );
    }
}

#[test]
fn compressible_formats_are_gzipped() {
    for mime_type in ["text/plain", "application/json", "application/pdf"] {
        assert!(
            !is_already_compressed(mime_type),
            "{mime_type} must be gzipped, got true"
        );
    }
}

#[test]
fn a_text_payload_is_gzipped_and_shrinks() {
    let payload = text_payload();

    let compressed =
        compress_for_storage(&payload, "text/plain").expect("compression failed");

    assert!(
        compressed.gzipped,
        "text must be recorded as compressed, got false"
    );
    assert!(
        compressed.bytes.len() < payload.len(),
        "gzip must shrink repetitive text, went from {} to {} bytes",
        payload.len(),
        compressed.bytes.len()
    );
}

#[test]
fn a_dense_payload_is_left_alone() {
    let payload = text_payload();

    let compressed =
        compress_for_storage(&payload, "application/zip").expect("compression failed");

    assert!(
        !compressed.gzipped,
        "an already compressed type must not be gzipped, got true"
    );
    assert_eq!(
        compressed.bytes, payload,
        "the bytes must pass through untouched, got {} bytes",
        compressed.bytes.len()
    );
}

#[test]
fn the_full_pipeline_round_trips() {
    let payload = text_payload();
    let master_key = encryption::generate_data_key();

    let compressed =
        compress_for_storage(&payload, "text/plain").expect("compression failed");
    let data_key = encryption::generate_data_key();
    let sealed = encryption::seal(&data_key, &compressed.bytes).expect("seal failed");
    let wrapped =
        encryption::wrap_data_key(&master_key, &data_key).expect("wrap failed");

    let recovered_key =
        encryption::unwrap_data_key(&master_key, &wrapped.ciphertext, &wrapped.nonce)
            .expect("unwrap failed");
    let opened = encryption::open(&recovered_key, &sealed.nonce, &sealed.ciphertext)
        .expect("open failed");
    let restored = handle_decompression(&opened, Compression::Gzip)
        .expect("decompression failed");

    assert_eq!(
        restored,
        payload,
        "the pipeline must give back the original bytes, got {} of {} bytes",
        restored.len(),
        payload.len()
    );
}

#[test]
fn stored_bytes_never_hold_the_plaintext() {
    let payload = text_payload();
    let compressed =
        compress_for_storage(&payload, "text/plain").expect("compression failed");
    let data_key = encryption::generate_data_key();

    let sealed = encryption::seal(&data_key, &compressed.bytes).expect("seal failed");

    assert!(
        !sealed
            .ciphertext
            .windows(payload.len().min(32))
            .any(|window| window == &payload[..payload.len().min(32)]),
        "the ciphertext must not contain the start of the plaintext, {} bytes stored",
        sealed.ciphertext.len()
    );
}

#[test]
fn storage_keys_leak_neither_name_nor_type() {
    let id = Uuid::now_v7();

    let content = content_key(id);
    let thumbnail = thumbnail_key(id);

    assert_eq!(
        content,
        format!("files/{id}/content"),
        "unexpected content key, got {content}"
    );
    assert_eq!(
        thumbnail,
        format!("files/{id}/thumbnail"),
        "unexpected thumbnail key, got {thumbnail}"
    );
    assert_ne!(
        content, thumbnail,
        "the two blobs of one file must not share a key, both were {content}"
    );
}
