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

fn test_rgba_png(path: &str, depth: u32) {
    let result = png_rgba::decode_png(path).unwrap();

    let max_value: usize = 2_usize.pow(depth) - 1;
    let offset = if depth == 16 {
        8
    } else {
        0
    };

    const H: usize = 64;
    const W: usize = 64;

    for y in 0..H {
        for x in 0..W {
            let r = x * max_value / (W - 1) >> offset;
            let g = y * max_value / (H - 1) >> offset;
            let b = max_value / 2 >> offset;
            let a = ((x + y) * max_value) / (W + H - 2) >> offset;

            assert!(r == result[y][4 * x] as usize);
            assert!(g == result[y][4 * x + 1] as usize);
            assert!(b == result[y][4 * x + 2] as usize);
            assert!(a == result[y][4 * x + 3] as usize);
        }
    }
}

#[test]
fn test_rgba_8bit_png() {
    test_rgba_png("tests/images/generated/rgba8-64x64.png", 8);
}

#[test]
fn test_rgba_16bit_png() {
    test_rgba_png("tests/images/generated/rgba16-64x64.png", 16);
}

fn test_rgb_png(path: &str, depth: u32) {
    let result = png_rgba::decode_png(path).unwrap();

    let max_value: usize = 2_usize.pow(depth) - 1;
    let offset = if depth == 16 {
        8
    } else {
        0
    };

    const H: usize = 64;
    const W: usize = 64;

    for y in 0..H {
        for x in 0..W {
            let r = x * max_value / (W - 1) >> offset;
            let g = y * max_value / (H - 1) >> offset;
            let b = max_value / 2 >> offset;
            let a = 255;

            assert!(r == result[y][4 * x] as usize);
            assert!(g == result[y][4 * x + 1] as usize);
            assert!(b == result[y][4 * x + 2] as usize);
            assert!(a == result[y][4 * x + 3] as usize);
        }
    }
}

#[test]
fn test_rgb_8bit_png() {
    test_rgb_png("tests/images/generated/rgb8-64x64.png", 8);
}

#[test]
fn test_rgb_16bit_png() {
    test_rgb_png("tests/images/generated/rgb16-64x64.png", 16);
}