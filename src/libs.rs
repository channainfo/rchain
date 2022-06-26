use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis()
}

pub fn u32_to_bytes(u: &u32) -> [u8; 4] {
    [
        ((u >> 24) & 0xff) as u8,
        (u >> 16 & 0xff) as u8,
        (u >> 8 & 0xff) as u8,
        (u & 0xff) as u8,
    ]
}

pub fn u64_to_bytes(u: &u64) -> [u8; 8] {
    [
        (u >> 56 & 0xff) as u8,
        (u >> 48 & 0xff) as u8,
        (u >> 40 & 0xff) as u8,
        (u >> 32 & 0xff) as u8,
        (u >> 24 & 0xff) as u8,
        (u >> 16 & 0xff) as u8,
        (u >> 8 & 0xff) as u8,
        (u & 0xff) as u8,
    ]
}

pub fn u128_to_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> 120 & 0xff) as u8,
        (u >> 112 & 0xff) as u8,
        (u >> 104 & 0xff) as u8,
        (u >> 96 & 0xff) as u8,
        (u >> 88 & 0xff) as u8,
        (u >> 80 & 0xff) as u8,
        (u >> 72 & 0xff) as u8,
        (u >> 64 & 0xff) as u8,
        (u >> 56 & 0xff) as u8,
        (u >> 48 & 0xff) as u8,
        (u >> 40 & 0xff) as u8,
        (u >> 32 & 0xff) as u8,
        (u >> 24 & 0xff) as u8,
        (u >> 16 & 0xff) as u8,
        (u >> 8 & 0xff) as u8,
        (u & 0xff) as u8,
    ]
}

pub fn difficulty_bytes_to_u8(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 0xf * 8)
        | ((v[30] as u128) << 0xe * 8)
        | ((v[29] as u128) << 0xd * 8)
        | ((v[28] as u128) << 0xc * 8)
        | ((v[27] as u128) << 0xb * 8)
        | ((v[26] as u128) << 0xa * 8)
        | ((v[25] as u128) << 0x9 * 8)
        | ((v[24] as u128) << 0x8 * 8)
        | ((v[23] as u128) << 0x7 * 8)
        | ((v[22] as u128) << 0x6 * 8)
        | ((v[21] as u128) << 0x5 * 8)
        | ((v[20] as u128) << 0x4 * 8)
        | ((v[19] as u128) << 0x3 * 8)
        | ((v[18] as u128) << 0x2 * 8)
        | ((v[17] as u128) << 0x1 * 8)
        | ((v[16] as u128) << 0x0 * 8)
}
