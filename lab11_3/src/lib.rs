fn encode_hex_nibble(nibble: u8) -> u8 {
    match nibble {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 2,
        4 => 6,
        5 => 7,
        6 => 5,
        7 => 4,
        8 => 0xC,
        9 => 0xD,
        0xA => 0xF,
        0xB => 0xE,
        0xC => 0xA,
        0xD => 0xB,
        0xE => 9,
        0xF => 8,
        a => panic!("Invalid hex value, {a}"),
    }
}
pub fn encode_hex(byte: u8) -> u8 {
    let low_nibble = byte & 0x0F;
    let high_nibble = byte >> 4;

    let encoded_low_nibble = encode_hex_nibble(low_nibble);
    let encoded_high_nibble = encode_hex_nibble(high_nibble);

    encoded_high_nibble << 4 | encoded_low_nibble
}

pub fn encode_hex_data(data: &[u8]) -> Vec<u8> {
    data.iter().copied().map(encode_hex).collect()
}

fn decode_hex_nibble(encoded: u8) -> u8 {
    match encoded {
        0 => 0,
        1 => 1,
        3 => 2,
        2 => 3,
        6 => 4,
        7 => 5,
        5 => 6,
        4 => 7,
        0xc => 8,
        0xd => 9,
        0xf => 0xa,
        0xe => 0xb,
        0xa => 0xc,
        0xb => 0xd,
        9 => 0xe,
        8 => 0xf,
        a => panic!("invalid hex value, {a}"),
    }
}

pub fn decode_hex(byte: u8) -> u8 {
    let low_nibble = byte & 0x0F;
    let high_nibble = byte >> 4;

    let decoded_low_nibble = decode_hex_nibble(low_nibble);
    let decoded_high_nibble = decode_hex_nibble(high_nibble);

    decoded_high_nibble << 4 | decoded_low_nibble
}

pub fn decode_hex_data(data: &[u8]) -> Vec<u8> {
    data.iter().copied().map(decode_hex).collect()
}

#[cfg(test)]
mod tests {
    use crate::{decode_hex, decode_hex_data, encode_hex, encode_hex_data};

    const FOX: &str = "The quick brown fox jumps over the lazy dog.";
    const ENCODED_DATA: &[u8] = &[
        0x76, 0x5C, 0x57, 0x30, 0x41, 0x47, 0x5D, 0x52, 0x5E, 0x30, 0x53, 0x43, 0x58, 0x44, 0x59,
        0x30, 0x55, 0x58, 0x4C, 0x30, 0x5F, 0x47, 0x5B, 0x40, 0x42, 0x30, 0x58, 0x45, 0x57, 0x43,
        0x30, 0x46, 0x5C, 0x57, 0x30, 0x5A, 0x51, 0x4F, 0x4D, 0x30, 0x56, 0x58, 0x54, 0x39,
    ];

    #[test]
    fn test_encode_hex() {
        assert_eq!(
            (0..16).map(encode_hex).collect::<Vec<_>>(),
            [0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4, 0xC, 0xD, 0xF, 0xE, 0xA, 0xB, 0x9, 0x8]
        );
        assert_eq!(encode_hex(0x54), 0x76);
        assert_eq!(encode_hex(0x68), 0x5C);
        let original_data = FOX.as_bytes();
        let encoded_data = ENCODED_DATA;
        assert_eq!(encode_hex_data(original_data), encoded_data);
    }

    #[test]
    fn test_decode_hex() {
        assert_eq!(
            (0..16).map(decode_hex).collect::<Vec<_>>(),
            [0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xF, 0xE, 0xC, 0xD, 0x8, 0x9, 0xB, 0xA]
        );
        assert_eq!(decode_hex(0x76), 0x54);
        assert_eq!(decode_hex(0x5C), 0x68);
        let original_data = FOX.as_bytes();
        let encoded_data = ENCODED_DATA;
        assert_eq!(decode_hex_data(encoded_data), original_data);
    }
}
