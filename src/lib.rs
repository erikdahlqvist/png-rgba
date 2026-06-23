use std::fs;

const PNG_SIGN: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

struct PngChunk {
    length: u32,
    chunk_type: u32,
    data: Vec<u8>,
    crc: u32,
}

pub fn png(path: &str) -> Result<Vec<Vec<u8>>, String> {
    let input = match fs::read(path) {
        Ok(input) => input,
        Err(_) => return Err(format!("Could not read file at: {path}")),
    };

    let sign_bytes = &input[..8];
    if sign_bytes != PNG_SIGN {
        return Err(String::from("Invalid PNG sign bytes"));
    }

    let mut chunks: Vec<PngChunk> = Vec::new();

    let mut i = 8;
    while i < input.len() {
        let length = u32::from_be_bytes(input[i..i + 4].try_into().unwrap());
        i += 4;

        let chunk_type = u32::from_be_bytes(input[i..i + 4].try_into().unwrap());
        i += 4;

        let data = input[i..i + length as usize].to_vec();
        i += length as usize;

        // TODO: verify checksum
        let crc = u32::from_be_bytes(input[i..i + 4].try_into().unwrap());
        i += 4;

        chunks.push(PngChunk { length, chunk_type, data, crc });
    }

    Ok(vec![]) // Temp return
}