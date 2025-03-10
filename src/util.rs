use std::string::FromUtf8Error;

pub fn cn_from_bytes(bytes: &[u8], offset: usize) -> Result<String, FromUtf8Error> {
    let length = bytes[offset] as usize;
    String::from_utf8(bytes[offset + 1..offset + 1 + length].to_vec())
}

pub fn bn_from_bytes(bytes: &[u8], offset: usize) -> Vec<u8> {
    let length = bytes[offset] as usize;
    bytes[offset + 1..offset + 1 + length].to_vec()
}
