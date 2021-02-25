
#![allow(clippy::unreadable_literal)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use core::iter;

mod error;

pub use crate::hex::error::FromHexError;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;
#[cfg(feature = "serde")]
pub use crate::serde::{deserialize, serialize, serialize_upper};


pub trait ToHex {
    fn encode_hex<T: iter::FromIterator<char>>(&self) -> T;

    fn encode_hex_upper<T: iter::FromIterator<char>>(&self) -> T;
}

const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

struct BytesToHexChars<'a> {
    inner: ::core::slice::Iter<'a, u8>,
    table: &'static [u8; 16],
    next: Option<char>,
}

impl<'a> BytesToHexChars<'a> {
    fn new(inner: &'a [u8], table: &'static [u8; 16]) -> BytesToHexChars<'a> {
        BytesToHexChars {
            inner: inner.iter(),
            table,
            next: None,
        }
    }
}

impl<'a> Iterator for BytesToHexChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(current) => Some(current),
            None => self.inner.next().map(|byte| {
                let current = self.table[(byte >> 4) as usize] as char;
                self.next = Some(self.table[(byte & 0xf) as usize] as char);
                current
            }),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.len();
        (length, Some(length))
    }
}

impl<'a> iter::ExactSizeIterator for BytesToHexChars<'a> {
    fn len(&self) -> usize {
        let mut length = self.inner.len() * 2;
        if self.next.is_some() {
            length += 1;
        }
        length
    }
}

fn encode_to_iter<T: iter::FromIterator<char>>(table: &'static [u8; 16], source: &[u8]) -> T {
    BytesToHexChars::new(source, table).collect()
}

impl<T: AsRef<[u8]>> ToHex for T {
    fn encode_hex<U: iter::FromIterator<char>>(&self) -> U {
        encode_to_iter(HEX_CHARS_LOWER, self.as_ref())
    }

    fn encode_hex_upper<U: iter::FromIterator<char>>(&self) -> U {
        encode_to_iter(HEX_CHARS_UPPER, self.as_ref())
    }
}


pub trait FromHex: Sized {
    type Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>;
}

fn val(c: u8, idx: usize) -> Result<u8, FromHexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(FromHexError::InvalidHexCharacter {
            c: c as char,
            index: idx,
        }),
    }
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let hex = hex.as_ref();
        if hex.len() % 2 != 0 {
            return Err(FromHexError::OddLength);
        }

        hex.chunks(2)
            .enumerate()
            .map(|(i, pair)| Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?))
            .collect()
    }
}

macro_rules! from_hex_array_impl {
    ($($len:expr)+) => {$(
        impl FromHex for [u8; $len] {
            type Error = FromHexError;

            fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
                let mut out = [0u8; $len];
                decode_to_slice(hex, &mut out as &mut [u8])?;
                Ok(out)
            }
        }
    )+}
}

from_hex_array_impl! {
    1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
    33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
    49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
    65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80
    81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96
    97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112
    113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128
    160 192 200 224 256 384 512 768 1024 2048 4096 8192 16384 32768
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
from_hex_array_impl! {
    65536 131072 262144 524288 1048576 2097152 4194304 8388608
    16777216 33554432 67108864 134217728 268435456 536870912
    1073741824 2147483648
}

#[cfg(target_pointer_width = "64")]
from_hex_array_impl! {
    4294967296
}

pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    data.encode_hex()
}

pub fn encode_upper<T: AsRef<[u8]>>(data: T) -> String {
    data.encode_hex_upper()
}

pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError> {
    FromHex::from_hex(data)
}

pub fn decode_to_slice<T: AsRef<[u8]>>(data: T, out: &mut [u8]) -> Result<(), FromHexError> {
    let data = data.as_ref();

    if data.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }
    if data.len() / 2 != out.len() {
        return Err(FromHexError::InvalidStringLength);
    }

    for (i, byte) in out.iter_mut().enumerate() {
        *byte = val(data[2 * i], 2 * i)? << 4 | val(data[2 * i + 1], 2 * i + 1)?;
    }

    Ok(())
}

fn generate_iter(len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..len).step_by(2).zip((0..len).skip(1).step_by(2))
}

// the inverse of `val`.
fn byte2hex(byte: u8, table: &[u8; 16]) -> (u8, u8) {
    let high = table[((byte & 0xf0) >> 4) as usize];
    let low = table[(byte & 0x0f) as usize];

    (high, low)
}

pub fn encode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<(), FromHexError> {
    if input.as_ref().len() * 2 != output.len() {
        return Err(FromHexError::InvalidStringLength);
    }

    for (byte, (i, j)) in input.as_ref().iter().zip(generate_iter(input.as_ref().len() * 2)) {
        let (high, low) = byte2hex(*byte, HEX_CHARS_LOWER);
        output[i] = high;
        output[j] = low;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_gen_iter() {
        let mut result = Vec::new();
        result.push((0, 1));
        result.push((2, 3));

        assert_eq!(generate_iter(5).collect::<Vec<_>>(), result);
    }

    #[test]
    fn test_encode_to_slice() {
        let mut output_1 = [0; 4 * 2];
        encode_to_slice(b"kiwi", &mut output_1).unwrap();
        assert_eq!(&output_1, b"6b697769");

        let mut output_2 = [0; 5 * 2];
        encode_to_slice(b"kiwis", &mut output_2).unwrap();
        assert_eq!(&output_2, b"6b69776973");

        let mut output_3 = [0; 100];

        assert_eq!(
            encode_to_slice(b"kiwis", &mut output_3),
            Err(FromHexError::InvalidStringLength)
        );
    }

    #[test]
    fn test_decode_to_slice() {
        let mut output_1 = [0; 4];
        decode_to_slice(b"6b697769", &mut output_1).unwrap();
        assert_eq!(&output_1, b"kiwi");

        let mut output_2 = [0; 5];
        decode_to_slice(b"6b69776973", &mut output_2).unwrap();
        assert_eq!(&output_2, b"kiwis");

        let mut output_3 = [0; 4];

        assert_eq!(decode_to_slice(b"6", &mut output_3), Err(FromHexError::OddLength));
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("foobar"), "666f6f626172");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("666f6f626172"), Ok(String::from("foobar").into_bytes()));
    }

    #[test]
    pub fn test_from_hex_okay_str() {
        assert_eq!(Vec::from_hex("666f6f626172").unwrap(), b"foobar");
        assert_eq!(Vec::from_hex("666F6F626172").unwrap(), b"foobar");
    }

    #[test]
    pub fn test_from_hex_okay_bytes() {
        assert_eq!(Vec::from_hex(b"666f6f626172").unwrap(), b"foobar");
        assert_eq!(Vec::from_hex(b"666F6F626172").unwrap(), b"foobar");
    }

    #[test]
    pub fn test_invalid_length() {
        assert_eq!(Vec::from_hex("1").unwrap_err(), FromHexError::OddLength);
        assert_eq!(Vec::from_hex("666f6f6261721").unwrap_err(), FromHexError::OddLength);
    }

    #[test]
    pub fn test_invalid_char() {
        assert_eq!(
            Vec::from_hex("66ag").unwrap_err(),
            FromHexError::InvalidHexCharacter { c: 'g', index: 3 }
        );
    }

    #[test]
    pub fn test_empty() {
        assert_eq!(Vec::from_hex("").unwrap(), b"");
    }

    #[test]
    pub fn test_from_hex_whitespace() {
        assert_eq!(
            Vec::from_hex("666f 6f62617").unwrap_err(),
            FromHexError::InvalidHexCharacter { c: ' ', index: 4 }
        );
    }

    #[test]
    pub fn test_from_hex_array() {
        assert_eq!(
            <[u8; 6] as FromHex>::from_hex("666f6f626172"),
            Ok([0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72])
        );

        assert_eq!(
            <[u8; 5] as FromHex>::from_hex("666f6f626172"),
            Err(FromHexError::InvalidStringLength)
        );
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(
            [0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72].encode_hex::<String>(),
            "666f6f626172".to_string(),
        );

        assert_eq!(
            [0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72].encode_hex_upper::<String>(),
            "666F6F626172".to_string(),
        );
    }
}