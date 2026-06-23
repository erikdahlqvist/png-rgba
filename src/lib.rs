use std::{fs, io::Read};

use flate2::read::ZlibDecoder;

const PNG_SIGN: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

struct PngChunk {
    length: u32,
    chunk_type: u32,
    data: Vec<u8>,
    crc: u32,
}

struct PngHeader {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    comp_type: u8,
    filter_type: u8,
    interl_type: u8,
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

    let header = extract_header(&chunks)?;

    let compressed_data = extract_data_segments(&chunks)?;
    let data = decompress_data(&compressed_data);

    Ok(vec![]) // Temp return
}

fn extract_header(chunks: &Vec<PngChunk>) -> Result<PngHeader, String> {
    let header_chunks: Vec<&PngChunk> = chunks.iter().filter(|c| c.chunk_type == u32::from_be_bytes(*b"IHDR")).collect();

    if header_chunks.is_empty() {
        return Err(String::from("No IHDR chunks where found"));
    } else if header_chunks.len() > 1 {
        return Err(String::from("More than one IHDR chunk was found"));
    }

    let header_chunk = header_chunks[0];
    if header_chunk.length != 13 {
        return Err(format!("IHDR length was {} bytes instead of expected 13 bytes", header_chunk.length));
    }

    let data = &header_chunk.data;

    Ok(PngHeader {
        width: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        height: u32::from_be_bytes(data[4..8].try_into().unwrap()),
        bit_depth: data[8],
        color_type: data[9],
        comp_type: data[10],
        filter_type: data[11],
        interl_type: data[12],
    })
}

fn extract_data_segments(chunks: &Vec<PngChunk>) -> Result<Vec<u8>, String> {
    let data_chunks: Vec<&PngChunk> = chunks.iter().filter(|c| c.chunk_type == u32::from_be_bytes(*b"IDAT")).collect();

    if data_chunks.is_empty() {
        return Err(String::from("Found no data chunks"));
    }

    Ok(data_chunks.iter().flat_map(|c| c.data.clone()).collect())
}

fn decompress_data(data: &Vec<u8>) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(data.as_slice());
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();

    decompressed
}