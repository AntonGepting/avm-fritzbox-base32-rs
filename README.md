# AVM Fritzbox Base32

[![Build Status](https://github.com/AntonGepting/avm-fritzbox-base32-rs/actions/workflows/actions.yml/badge.svg)](https://github.com/AntonGepting/avm-fritzbox-base32-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/avm-fritzbox-base32.svg)](https://crates.io/crates/avm_fritzbox-base32)
[![Documentation](https://docs.rs/avm_fritzbox-base32/badge.svg)](https://docs.rs/avm_fritzbox-base32)


## Description

`avm-fritzbox-base32` library povides base32 encode/decode functions, which are
used for decoding exported data from avm fritzbox.

Current limitations:
* different alphabet (`ABCDEFGHIJKLMNOPQRSTUVWXYZ123456`)
* padding not supported (usually base encodings use `=` for padding)
* encoding input data must be aligned to 5 bytes
* decoding input string must be aligned to 8 chars


## Quick Start

* Add the new crate dependency in your `Cargo.toml`.

    ```
    [dependencies]
    avm-fritzbox-base32 = "0.1.0"
    ```

* Encode data

    ```
    extern crate avm_fritzbox_base32;

    fn main() {
        let data = b"Hello, World!!!";
        let s = avm_fritzbox_base32::encode(&data).unwrap();
    }
    ```

* Decode string

    ```
    extern crate avm_fritzbox_base32;

    fn main() {
        let s = "JBSWY2DPFQQFO22SNRSCCIJB";
        let data = avm_fritzbox_base32::decode(s).unwrap();
    }
    ```


## See Also:

* [Rust Base32 Crate](https://crates.io/crates/base32)
* [PeterPawn's Decoder](https://github.com/PeterPawn/decoder)
* [RFC4648 - The Base16, Base32, and Base64 Data Encodings](https://datatracker.ietf.org/doc/html/rfc4648)

