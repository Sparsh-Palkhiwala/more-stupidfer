#![allow(non_snake_case)]

pub fn U1(bytes: &[u8], offset: &mut usize) -> u8 {
    let x = bytes[*offset];
    *offset += 1;
    x
}

pub fn U2(bytes: &[u8], offset: &mut usize) -> u16 {
    let x = u16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap());
    *offset += 2;
    x
}

pub fn U4(bytes: &[u8], offset: &mut usize) -> u32 {
    let x = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    x
}

pub fn I1(bytes: &[u8], offset: &mut usize) -> i8 {
    let x = bytes[*offset] as i8;
    *offset += 1;
    x
}

pub fn I2(bytes: &[u8], offset: &mut usize) -> i16 {
    let x = i16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap());
    *offset += 2;
    x
}

pub fn I4(bytes: &[u8], offset: &mut usize) -> i32 {
    let x = i32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    x
}
pub fn R4(bytes: &[u8], offset: &mut usize) -> f32 {
    let x = f32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    x
}

pub fn C1(bytes: &[u8], offset: &mut usize) -> char {
    let x = char::from_u32(bytes[*offset] as u32)
        .expect("Failed to parse C1 from {offset} from\n{bytes:#?}");
    *offset += 1;
    x
}

pub fn Cn(bytes: &[u8], offset: &mut usize) -> String {
    let length = bytes[*offset] as usize;
    let result = String::from_utf8(bytes[*offset + 1..*offset + 1 + length].to_vec());
    if let Ok(s) = result {
        *offset += 1 + length;
        s
    } else {
        panic!("Failed to parse Cn from {offset} with length {length} from\n{bytes:#?}");
    }
}

pub fn Bn(bytes: &[u8], offset: &mut usize) -> Vec<u8> {
    let length = bytes[*offset] as usize;
    let x = bytes[*offset + 1..*offset + 1 + length].to_vec();
    *offset += 1 + length;
    x
}

pub fn Dn(bytes: &[u8], offset: &mut usize) -> Vec<u8> {
    let nbits = u16::from_le_bytes(bytes[*offset..*offset + 2].try_into().unwrap()) as usize;
    let length = nbits.div_ceil(8);
    let dn = bytes[*offset + 1..*offset + 1 + length].to_vec();
    *offset += 2 + length;
    dn
}

pub fn kxU1(contents: &[u8], num: usize, offset: &mut usize) -> Vec<u8> {
    let x = contents[*offset..*offset + num].to_vec();
    *offset += num;
    x
}

pub fn kxU2(contents: &[u8], num: u16, offset: &mut usize) -> Vec<u16> {
    let mut v = Vec::with_capacity(num as usize);
    for _ in 0..num {
        let x = u16::from_le_bytes(contents[*offset..*offset + 2].try_into().unwrap());
        v.push(x);
        *offset += 2;
    }
    v
}

pub fn kxN1(contents: &[u8], num: u16, offset: &mut usize) -> Vec<u8> {
    let nbytes = num.div_ceil(2) as usize;
    let mut v = Vec::with_capacity(num as usize);
    for _ in 0..nbytes {
        let x = contents[*offset];
        v.push((x >> 0) & 0xf); // lower nibble
        v.push((x >> 4) & 0xf); // upper nibble
        *offset += 1;
    }
    v
}
