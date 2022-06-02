use super::common::*;

fn bit_quintet_to_char(d: u8) -> Option<char> {
    ALPHABET.chars().nth(d as usize)
}

// TODO: explanaition
fn encode_5_bytes(buf: &[u8; 5]) -> Option<String> {
    let mut s = String::new();
    // example:
    // buf = [0b1111_1111, 0b1111_1111, ..., 0b1111_1111; 5] (+ residue 0b0000_0000)
    //
    // intermediate result:
    // s[0] = bin_quintet_to_char(0b0001_1111) = '6' (+ res 0b0000_0111)
    s.push(bit_quintet_to_char(buf[0] >> 3)?);
    // s[1] = bin_quintet_to_char(0b0001_1100 | 0b0000_0011) = 0b0001_1111 = '6' (+ res 0b0011_1111)
    s.push(bit_quintet_to_char(
        ((buf[0] & 0b000_0111) << 2) | (buf[1] >> 6),
    )?);
    // s[2] = bin_quintet_to_char(0b0001_1111) = '6' (+ res 0b0000_0001)
    s.push(bit_quintet_to_char((buf[1] & 0b0011_1110) >> 1)?);
    // s[3] = bin_quintet_to_char(0b0001_0000 | 0b0000_1111) = 0b0001_1111 = '6' (+ res 0b0000_1111)
    s.push(bit_quintet_to_char(
        ((buf[1] & 0b0000_0001) << 4) | (buf[2] >> 4),
    )?);
    // s[4] = bin_quintet_to_char(0b0001_0000 | 0b0000_1111) = 0b0001_1111 = '6' (+ res 0b0111_1111)
    s.push(bit_quintet_to_char(
        ((buf[2] & 0b0000_1111) << 1) | (buf[3] >> 7),
    )?);
    // s[5] = bin_quintet_to_char(0b0001_1111) = 0b0001_1111 = '6' (+ res 0b0000_0011)
    s.push(bit_quintet_to_char((buf[3] & 0b0111_1100) >> 2)?);
    // s[6] = bin_quintet_to_char(0b0001_1000 | 0b0000_0111) = 0b0001_1111 = '6' (+ res 0b0001_1111)
    s.push(bit_quintet_to_char(
        ((buf[3] & 0b0000_0011) << 3) | (buf[4] >> 5),
    )?);
    // s[7] = bin_quintet_to_char(0b0001_1111) = 0b0001_1111 = '6' (+ res 0b0000_0000)
    s.push(bit_quintet_to_char(buf[4] & 0b0001_1111)?);

    Some(s)
}

// NOTE: input 5 bytes align (5, 15, 20, ... 50 ...)
// no traditional padding supported `=`
pub fn encode(input: &[u8]) -> Option<String> {
    let mut s = String::new();

    for chunk in input.chunks(5) {
        let mut buf = [0u8; 5];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b
        }

        // 5 bytes into 8 chars
        s += &encode_5_bytes(&buf)?;
    }

    Some(s)
}

#[test]
fn encode_test() {
    let orig = "EWOYB6UU4CXNGTRIIFPBKROUTTMBZ53XATBLWTXHOFXRTO2UVC3I32RS132MAAC3BXGSM6HGYS51S4W1";
    let data = [
        0x25, 0x9d, 0x80, 0xfe, 0x94, 0xe8, 0xae, 0xd3, 0x4e, 0x28, 0x41, 0x5e, 0x15, 0x45, 0xd4,
        0x9c, 0xd8, 0x1c, 0xfb, 0x97, 0x04, 0xc2, 0xbb, 0x4e, 0xe7, 0x71, 0x6f, 0x19, 0xbb, 0x74,
        0xa8, 0xb8, 0x8e, 0x6e, 0x32, 0xd7, 0x36, 0xc0, 0x00, 0x5c, 0x0d, 0xcd, 0x26, 0x7c, 0xe6,
        0xc4, 0xbd, 0xa9, 0x76, 0xda,
    ];
    let s = encode(&data).unwrap();
    assert_eq!(s, orig);

    let orig = "J4U52P1Y3JBDZWIXNCVQRVG2B63N4TJLJO456FTQNNOEUGJEQWXEXWHIOZBNYDTVI51X3CNLNF15W4W1";
    let data = [
        0x4f, 0x69, 0xed, 0xbf, 0x58, 0xe2, 0x42, 0x3c, 0xd9, 0x17, 0x68, 0xab, 0x08, 0xd4, 0xdb,
        0x0f, 0xf8, 0xde, 0xcd, 0x2b, 0x4b, 0xbb, 0xef, 0x96, 0x70, 0x6b, 0x5c, 0x4a, 0x19, 0x24,
        0x85, 0xae, 0x4b, 0xd8, 0xe8, 0x76, 0x42, 0xdc, 0x0e, 0x75, 0x47, 0xb5, 0x7e, 0x09, 0xab,
        0x69, 0x75, 0xeb, 0x76, 0xda,
    ];
    let s = encode(&data).unwrap();
    assert_eq!(s, orig);

    let orig = "66666666666666666666666666666666666666666666666666666666666666666666666666666666";
    let data = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff,
    ];
    let s = encode(&data).unwrap();
    assert_eq!(s, orig);
}
