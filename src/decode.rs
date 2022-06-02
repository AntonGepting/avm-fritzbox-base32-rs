use super::common::*;

/// convert char to binary representation
// long:
// decode char into bin using interval, defined by first char code and offset
// e.g.
// alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";
// c = 'A': first = 'A', offset = 0,  result = 0  = 0b0000_0000;
// c = '1': first = '1', offset = 26, result = 26 = 0b0001_1010;
// c = '2': first = '1', offset = 26, result = 27 = 0b0001_1011;
fn char_to_bin(c: char, first: char, offset: u8) -> u8 {
    c as u8 - first as u8 + offset
}

// convert to 5 bit representation
//
//const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";
// [26 letters A..Z][6 numbers 1..6]
//
// out: 5 lowest bits only
// (0b0000_0000..0b0001_1111 = 0..31)
fn char_to_bit_quintet(c: char) -> Option<u8> {
    // if char is in chars interval (between first char code and last items)
    // return it's converted bin value, if no errors
    let result = match c {
        FIRST_LETTER..=LAST_LETTER => char_to_bin(c, FIRST_LETTER, LETTERS_OFFSET),
        FIRST_NUMBER..=LAST_NUMBER => char_to_bin(c, FIRST_NUMBER, NUMBERS_OFFSET),
        _ => return None,
    };

    // check if it is out 5 bits bounds
    //if result & 0b0001_1111 != result {
    //return None;
    //};

    Some(result)
}

#[test]
fn char_to_bit_quintet_test() {
    let a = char_to_bit_quintet('A').unwrap();
    let z = char_to_bit_quintet('Z').unwrap();
    let n1 = char_to_bit_quintet('1').unwrap();
    let n6 = char_to_bit_quintet('6').unwrap();

    assert_eq!(a, 0);
    assert_eq!(z, 25);
    assert_eq!(n1, 26);
    assert_eq!(n6, 31);
}

fn decode_8_chars(buf: &[u8; 8]) -> Vec<u8> {
    let mut data = Vec::new();
    // there are 8 significant bits quintets
    // they will be merged per byte and pushed in the line of bytes from left to right
    // residue - remaining bits, after cutoff, using non-circular shifting left or right
    // result - resulting binary data
    //
    // example: buf = [0b00001_1111, 0b00001_1111, ..., 0b00001_1111; 8] (+ residue = 0)
    //
    // intermediate result step by step:
    // data[0]= 0b1111_1000 | 0b0000_0111 = 0b1111_1111 (+res = 0b1100_0000)
    data.push((buf[0] << 3) | (buf[1] >> 2));
    // data[1]= 0b1100_0000 | 0b0011_1110 | 0b0000_0001 = 0b1111_1111 (+ res = 0b1111_0000)
    data.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
    // data[2]= 0b1111_0000 | 0b0000_1111 = 0b1111_1111 (+ res = 0b1000_0000)
    data.push((buf[3] << 4) | (buf[4] >> 1));
    // data[3]= 0b1000_0000 | 0b0111_1100 | 0b0000_0011 = 0b1111_1111 (+ res 0b1110_0000)
    data.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
    // data[4]= 0b1110_0000 | 0b0001_1111 = 0b1111_1111 (+ res 0)
    data.push((buf[6] << 5) | buf[7]);
    // data = [0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111] (+ res = 0)
    data
}

//
// ascii custom char table:
// [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
//  1, 2, 3, 4, 5, 6]
// equivalent dec number table
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,
// 26,27,28,29,30,31]
//
// (e.g. 'A' = 0x41 from char table is equivalent 0 from dec number table)
//
// result number for one char is between 0..31
// 0..31 = 32 = 5 bit for coding
//
// can be coded with 5 bits = 4..0 bits
// [2^4  2^3  2^2  2^1  2^0] (4..0) = 5 bit
// [ 16    8    4    2    1] max number coded by bit
//
// cant access by bits, only byte (8 bit)
//
// 1. split string in 8 chars chunks
// 2. decode each char from chank into bin quintet (u8 containing only 5 bits information)
// 3. merge quintets into 5 bytes of data and store it as part of result
// 4. return resulting array of data
/// decode avm base32 encoded string
// NOTE: input 8 bytes align (8, 16, 24, ... 80 ...)
// no traditional padding `=` supported
pub fn decode(input: &str) -> Option<Vec<u8>> {
    let mut data: Vec<u8> = Vec::new();

    // split str into 8 chars chunks
    for chunk in input.as_bytes().chunks(8) {
        let mut buf = [0u8; 8]; // = [0, 0, 0, 0, 0, 0, 0, 0];

        // for every char in current chunk of 8 chars get and store bits quintet (0b0001_1111)
        for (i, c) in chunk.iter().enumerate() {
            buf[i] = char_to_bit_quintet(*c as char)?;
        }

        // convert 8 bit_quintets to 5 bytes and store
        data.extend(decode_8_chars(&buf));
    }

    Some(data)
}

#[test]
fn decode_test() {
    let s = "EWOYB6UU4CXNGTRIIFPBKROUTTMBZ53XATBLWTXHOFXRTO2UVC3I32RS132MAAC3BXGSM6HGYS51S4W1";
    let orig = [
        0x25, 0x9d, 0x80, 0xfe, 0x94, 0xe8, 0xae, 0xd3, 0x4e, 0x28, 0x41, 0x5e, 0x15, 0x45, 0xd4,
        0x9c, 0xd8, 0x1c, 0xfb, 0x97, 0x04, 0xc2, 0xbb, 0x4e, 0xe7, 0x71, 0x6f, 0x19, 0xbb, 0x74,
        0xa8, 0xb8, 0x8e, 0x6e, 0x32, 0xd7, 0x36, 0xc0, 0x00, 0x5c, 0x0d, 0xcd, 0x26, 0x7c, 0xe6,
        0xc4, 0xbd, 0xa9, 0x76, 0xda,
    ];
    let data = decode(s).unwrap();
    assert_eq!(data, orig);

    let s = "J4U52P1Y3JBDZWIXNCVQRVG2B63N4TJLJO456FTQNNOEUGJEQWXEXWHIOZBNYDTVI51X3CNLNF15W4W1";
    let orig = [
        0x4f, 0x69, 0xed, 0xbf, 0x58, 0xe2, 0x42, 0x3c, 0xd9, 0x17, 0x68, 0xab, 0x08, 0xd4, 0xdb,
        0x0f, 0xf8, 0xde, 0xcd, 0x2b, 0x4b, 0xbb, 0xef, 0x96, 0x70, 0x6b, 0x5c, 0x4a, 0x19, 0x24,
        0x85, 0xae, 0x4b, 0xd8, 0xe8, 0x76, 0x42, 0xdc, 0x0e, 0x75, 0x47, 0xb5, 0x7e, 0x09, 0xab,
        0x69, 0x75, 0xeb, 0x76, 0xda,
    ];
    let data = decode(s).unwrap();
    assert_eq!(data, orig);

    let s = "66666666666666666666666666666666666666666666666666666666666666666666666666666666";
    let orig = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff,
    ];
    let data = decode(s).unwrap();
    assert_eq!(data, orig);
}
