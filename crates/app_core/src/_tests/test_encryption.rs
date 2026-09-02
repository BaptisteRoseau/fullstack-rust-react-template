use super::*;

const PLAINTEXT: &[u8] = b"the quick brown fox jumps over the lazy dog";

#[test]
fn sealing_then_opening_gives_back_the_original_bytes() {
    let key = generate_data_key();

    let sealed = seal(&key, PLAINTEXT).expect("seal failed");
    let opened = open(&key, &sealed.nonce, &sealed.ciphertext).expect("open failed");

    assert_eq!(
        opened,
        PLAINTEXT,
        "the round trip changed the bytes, got {:?}",
        String::from_utf8_lossy(&opened)
    );
}

#[test]
fn the_ciphertext_does_not_contain_the_plaintext() {
    let key = generate_data_key();

    let sealed = seal(&key, PLAINTEXT).expect("seal failed");

    assert_ne!(
        sealed.ciphertext, PLAINTEXT,
        "the ciphertext is the plaintext, length {}",
        sealed.ciphertext.len()
    );
    assert_eq!(
        sealed.ciphertext.len(),
        PLAINTEXT.len() + 16,
        "expected the plaintext plus a 16-byte GCM tag, got {} bytes",
        sealed.ciphertext.len()
    );
}

#[test]
fn opening_with_the_wrong_key_fails() {
    let key = generate_data_key();
    let other_key = generate_data_key();
    let sealed = seal(&key, PLAINTEXT).expect("seal failed");

    let opened = open(&other_key, &sealed.nonce, &sealed.ciphertext);

    assert!(
        opened.is_err(),
        "opening under a different key must fail, got {:?} bytes back",
        opened.map(|bytes| bytes.len())
    );
}

#[test]
fn opening_with_the_wrong_nonce_fails() {
    let key = generate_data_key();
    let sealed = seal(&key, PLAINTEXT).expect("seal failed");
    let other_nonce = generate_nonce();

    let opened = open(&key, &other_nonce, &sealed.ciphertext);

    assert!(
        opened.is_err(),
        "opening under a different nonce must fail, got {:?} bytes back",
        opened.map(|bytes| bytes.len())
    );
}

#[test]
fn opening_with_a_nonce_of_the_wrong_length_fails() {
    let key = generate_data_key();
    let sealed = seal(&key, PLAINTEXT).expect("seal failed");

    let opened = open(&key, &sealed.nonce[..NONCE_LENGTH - 1], &sealed.ciphertext);

    assert!(
        opened.is_err(),
        "a short nonce must be refused, got {:?} bytes back",
        opened.map(|bytes| bytes.len())
    );
}

#[test]
fn opening_tampered_ciphertext_fails() {
    let key = generate_data_key();
    let mut sealed = seal(&key, PLAINTEXT).expect("seal failed");
    sealed.ciphertext[0] ^= 0xFF;

    let opened = open(&key, &sealed.nonce, &sealed.ciphertext);

    assert!(
        opened.is_err(),
        "a flipped byte must fail authentication, got {:?} bytes back",
        opened.map(|bytes| bytes.len())
    );
}

#[test]
fn two_seals_of_the_same_bytes_differ() {
    let key = generate_data_key();

    let first = seal(&key, PLAINTEXT).expect("first seal failed");
    let second = seal(&key, PLAINTEXT).expect("second seal failed");

    assert_ne!(
        first.nonce, second.nonce,
        "each seal must draw its own nonce, both were {:?}",
        first.nonce
    );
    assert_ne!(
        first.ciphertext, second.ciphertext,
        "two seals of the same bytes must not be identical, length {}",
        first.ciphertext.len()
    );
}

#[test]
fn a_wrapped_data_key_round_trips() {
    let master_key = generate_data_key();
    let data_key = generate_data_key();

    let wrapped = wrap_data_key(&master_key, &data_key).expect("wrap failed");
    let unwrapped =
        unwrap_data_key(&master_key, &wrapped.ciphertext, &wrapped.nonce)
            .expect("unwrap failed");

    assert_eq!(
        unwrapped, data_key,
        "the unwrapped key differs from the original, got {unwrapped:?}"
    );
}

#[test]
fn unwrapping_under_a_different_master_key_fails() {
    let master_key = generate_data_key();
    let other_master_key = generate_data_key();
    let data_key = generate_data_key();
    let wrapped = wrap_data_key(&master_key, &data_key).expect("wrap failed");

    let unwrapped =
        unwrap_data_key(&other_master_key, &wrapped.ciphertext, &wrapped.nonce);

    assert!(
        unwrapped.is_err(),
        "a different master key must not unwrap, got {unwrapped:?}"
    );
}

#[test]
fn a_payload_sealed_with_an_explicit_nonce_opens_with_it() {
    let key = generate_data_key();
    let nonce = generate_nonce();

    let ciphertext =
        seal_with_nonce(&key, &nonce, PLAINTEXT).expect("seal_with_nonce failed");
    let opened = open(&key, &nonce, &ciphertext).expect("open failed");

    assert_eq!(
        opened,
        PLAINTEXT,
        "the round trip changed the bytes, got {:?}",
        String::from_utf8_lossy(&opened)
    );
}

#[test]
fn an_empty_payload_round_trips() {
    let key = generate_data_key();

    let sealed = seal(&key, b"").expect("seal failed");
    let opened = open(&key, &sealed.nonce, &sealed.ciphertext).expect("open failed");

    assert!(
        opened.is_empty(),
        "an empty payload must come back empty, got {} bytes",
        opened.len()
    );
}
