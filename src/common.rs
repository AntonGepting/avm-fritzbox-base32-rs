// XXX: hold alphabet as bytes or chars string?
//const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";
pub(crate) const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";

//  letters offset            numbers offset
//  v                         v
// "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456"
//  ^                        ^^    ^
//  |                        ||    last number
//  |                        |first number
//  first letter             last letter
pub(crate) const FIRST_LETTER: char = 'A';
pub(crate) const LAST_LETTER: char = 'Z';
pub(crate) const LETTERS_OFFSET: u8 = 0;

pub(crate) const FIRST_NUMBER: char = '1';
pub(crate) const LAST_NUMBER: char = '6';
pub(crate) const NUMBERS_OFFSET: u8 = 26;
