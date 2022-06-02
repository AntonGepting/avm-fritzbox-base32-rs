// XXX: hold alphabet as bytes or chars string?
//const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";
pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";

//  letters offset            numbers offset
//  v                         v
// "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456"
//  ^                        ^^    ^
//  |                        ||    last number
//  |                        |first number
//  first letter             last letter
pub const FIRST_LETTER: char = 'A';
pub const LAST_LETTER: char = 'Z';
pub const LETTERS_OFFSET: u8 = 0;

pub const FIRST_NUMBER: char = '1';
pub const LAST_NUMBER: char = '6';
pub const NUMBERS_OFFSET: u8 = 26;
