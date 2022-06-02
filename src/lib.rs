pub mod common;
pub mod decode;
pub mod encode;

pub use decode::decode;
pub use encode::encode;

#[test]
fn encode_decode() {
    use crate::{decode, encode};

    let data_orig = b"Hello, World!!!";
    let s_orig = "JBSWY2DPFQQFO22SNRSCCIJB";

    let s = encode(data_orig).unwrap();
    assert_eq!(s_orig, s);

    let data = decode(&s).unwrap();
    assert_eq!(data_orig.to_vec(), data);
}
