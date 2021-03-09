#![allow(unused)]

use std::io::Error;
use neo_crypto::hex;

/**
 * XORs two hexstrings
 * @param str1 HEX string
 * @param str2 HEX string
 * @returns XOR output as a HEX string
 */
pub fn hex_xor<'a>(str1: &'a str, str2: &'a str) -> Result<String, Error> {
    if str1.len() != str2.len() {
        ()
    }

    let mut v1 = hex::decode(str1).unwrap();
    let mut v2 = hex::decode(str2).unwrap();

    for i in 0..v1.len() {
        v1[i] = v1[i]^v2[i];
    }

    Ok(hex::encode(v1))
}


/**
 * Reverses an array.
 * @example
 * reverse_array('abcd') = 'cdba'
 */
pub fn reverse_array<T>(arr: &mut [T]) -> Result<&[T], Error> {

    arr.reverse();
    Ok(arr)

}

/**
 * Reverses a HEX string, treating 2 chars as a byte.
 * @example
 * reverse_hex('abcdef') = 'efcdab'
 */
pub fn reverse_hex(hex: &str) -> String {

    let mut value = hex::decode(hex).unwrap();
    value.reverse();

    hex::encode(value)
}


#[cfg(test)]
mod tests {
    use crate::misc::{reverse_array, reverse_hex, hex_xor};

    #[test]
    pub fn test_reverse_arr(){
        let mut arr = ['a','b','c','d'];
         reverse_array(&mut arr);
        assert_eq!(arr[0], 'd');
    }

    #[test]
    pub fn test_reverse_hex(){

        let rev = reverse_hex("fd2c2b");
        assert_eq!("2b2cfd", rev);
    }

    #[test]
    pub fn test_hex_xor(){
        let hex_1 = "fd2c2b414e81";
        let hex_2 = "dd71004ffc93";

        let res =  hex_xor(&hex_1, &hex_2).unwrap();

        assert_eq!(res,"205d2b0eb212")

    }
}