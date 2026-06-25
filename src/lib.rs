use std::{fs, io::Read};

use flate2::read::ZlibDecoder;

mod error;
pub use crate::error::Error::{self, *};

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

pub fn decode_png(path: &str) -> Result<Vec<Vec<u8>>, Error> {
    let input = match fs::read(path) {
        Ok(input) => input,
        Err(_) => return Err(CouldNotReadFile),
    };

    let sign_bytes = &input[..8];
    if sign_bytes != PNG_SIGN {
        return Err(InvalidPngSignature);
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

        let crc = u32::from_be_bytes(input[i..i + 4].try_into().unwrap());
        if crc != crc32(&input[i - 4 - length as usize..i]) {
            return Err(InvalidChecksum);
        }
        i += 4;

        chunks.push(PngChunk { length, chunk_type, data, crc });
    }

    let header = extract_header(&chunks)?;

    let compressed_data = extract_data_segments(&chunks)?;
    let data = decompress_data(&compressed_data);

    let output = defilter(&data, &header)?;

    Ok(output)
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF_u32;

    for &byte in data {
        crc ^= byte as u32;

        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB88320 & (0u32.wrapping_sub(crc & 1)));
        }
    }

    !crc
}

fn extract_header(chunks: &Vec<PngChunk>) -> Result<PngHeader, Error> {
    let header_chunks: Vec<&PngChunk> = chunks.iter().filter(|c| c.chunk_type == u32::from_be_bytes(*b"IHDR")).collect();

    if header_chunks.is_empty() {
        return Err(NoHeaderChunkFound);
    } else if header_chunks.len() > 1 {
        return Err(MultipleHeaderChunksFound);
    }

    let header_chunk = header_chunks[0];
    if header_chunk.length != 13 {
        return Err(HeaderLengthNot13);
    }

    let data = &header_chunk.data;

    let header = PngHeader {
        width: u32::from_be_bytes(data[0..4].try_into().unwrap()),
        height: u32::from_be_bytes(data[4..8].try_into().unwrap()),
        bit_depth: data[8],
        color_type: data[9],
        comp_type: data[10],
        filter_type: data[11],
        interl_type: data[12],
    };

    if header.comp_type != 0 {
        return Err(NonZeroCompressionType);
    } else if header.filter_type != 0 {
        return Err(NonZeroFilterType);
    } else if header.interl_type > 1 {
        return Err(InterlacingTypeGreaterThanOne);
    }

    Ok(header)
}

fn extract_data_segments(chunks: &Vec<PngChunk>) -> Result<Vec<u8>, Error> {
    let data_chunks: Vec<&PngChunk> = chunks.iter().filter(|c| c.chunk_type == u32::from_be_bytes(*b"IDAT")).collect();

    if data_chunks.is_empty() {
        return Err(NoDataChunksFound);
    }

    Ok(data_chunks.iter().flat_map(|c| c.data.clone()).collect())
}

fn decompress_data(data: &Vec<u8>) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(data.as_slice());
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();

    decompressed
}

fn defilter(data: &Vec<u8>, header: &PngHeader) -> Result<Vec<Vec<u8>>, Error> {
    // TODO: make defilter work for non RGBA
    let bits_per_pixel = header.bit_depth * 4;
    let bytes_per_scanline = header.width * bits_per_pixel as u32 / 8 + 1;

    let mut unfiltered: Vec<Vec<u8>> = Vec::new();
    for (i, scanline) in data.chunks(bytes_per_scanline as usize).enumerate() {
        let filter = scanline[0];

        let mut row: Vec<u8> = Vec::with_capacity(scanline.len() - 1);

        for j in 0..scanline.len() - 1 {
            let left = if j < 4 {
                0
            } else {
                row[j - 4]
            };
            let up = if unfiltered.is_empty() {
                0
            } else {
                unfiltered[i - 1][j]
            };
            let up_left = if unfiltered.is_empty() || j < 4 {
                0
            } else {
                unfiltered[i - 1][j - 4]
            };

            row.push(match filter {
                0 => scanline[j + 1],
                1 => scanline[j + 1].wrapping_add(left),
                2 => scanline[j + 1].wrapping_add(up),
                3 => scanline[j + 1].wrapping_add(((left as u16 + up as u16) / 2) as u8),
                4 => scanline[j + 1].wrapping_add(paeth(left, up, up_left)),
                x => return Err(UnrecognizedFilterType(x)),
            });
        }

        unfiltered.push(row);
    }

    Ok(unfiltered)
}

fn paeth(left: u8, up: u8, up_left: u8) -> u8 {
    let p: i16 = left as i16 + up as i16 - up_left as i16;
    let pa = p.abs_diff(left as i16);
    let pb = p.abs_diff(up as i16);
    let pc = p.abs_diff(up_left as i16);

    if pa <= pb && pa <= pc {
        left
    } else if pb <= pc {
        up
    } else {
        up_left
    }
}