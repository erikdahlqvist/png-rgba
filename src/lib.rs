use std::fs;

const PNG_SIGN: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

pub fn png(path: &str) -> Result<Vec<Vec<u8>>, String> {
    let input = match fs::read(path) {
        Ok(input) => input,
        Err(_) => return Err(format!("Could not read file at: {path}")),
    };

    let sign_bytes = &input[..8];
    if sign_bytes != PNG_SIGN {
        return Err(String::from("Invalid PNG sign bytes"));
    }

    Ok(vec![]) // Temp return
}