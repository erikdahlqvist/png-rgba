use png_rgba::{self, *};

#[test]
fn test_file_not_found() {
    let result = png_rgba::decode_png("tests/images/file-not-found.png");
    assert!(result == Err(CouldNotReadFile))
}

#[test]
fn invalid_png_signature() {
    let result = png_rgba::decode_png("tests/images/invalid-signature.png");
    assert!(result == Err(InvalidPngSignature))
}

#[test]
fn test_checksum_invalid_png() {
    let result = png_rgba::decode_png("tests/images/invalid-checksum.png");
    assert!(result == Err(InvalidChecksum))
}