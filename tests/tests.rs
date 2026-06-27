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

#[test]
fn test_rgba_8bit_png() {
    let result = png_rgba::decode_png("tests/images/generated/rgba8-64x64.png").unwrap();

    const H: usize = 64;
    const W: usize = 64;

    for y in 0..H {
        for x in 0..W {
            let r = x * 255 / (W - 1);
            let g = y * 255 / (H - 1);
            let b = 127;
            let a = ((x + y) * 255) / (W + H - 2);

            assert!(r == result[y][4 * x] as usize);
            assert!(g == result[y][4 * x + 1] as usize);
            assert!(b == result[y][4 * x + 2] as usize);
            assert!(a == result[y][4 * x + 3] as usize);
        }
    }
}

#[test]
fn test_rgba_16bit_png() {
    let result = png_rgba::decode_png("tests/images/generated/rgba16-64x64.png").unwrap();

    const H: usize = 64;
    const W: usize = 64;

    for y in 0..H {
        for x in 0..W {
            let r = x * 65535 / (W - 1) >> 8;
            let g = y * 65535 / (H - 1) >> 8;
            let b = 65535 / 2 >> 8;
            let a = ((x + y) * 65535) / (W + H - 2) >> 8;

            assert!(r == result[y][4 * x] as usize);
            assert!(g == result[y][4 * x + 1] as usize);
            assert!(b == result[y][4 * x + 2] as usize);
            assert!(a == result[y][4 * x + 3] as usize);
        }
    }
}