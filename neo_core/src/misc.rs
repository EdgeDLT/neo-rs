use std::io::Error;
use neo_crypto::hex;

/**
 * XORs two hexstrings
 * @param str1 HEX string
 * @param str2 HEX string
 * @returns XOR output as a HEX string
 */
pub fn hexXor(str1: &str, str2: &str) -> Result<&str, Error> {
    if str1.len() != str2.len() {
        Err(())
    }

    let mut v1 = hex::decode(str1).unwrap();
    let mut v2 = hex::decode(str2).unwrap();

    for i in 0..v1.len() {
        v1[i] = v1[i]^v2[i];
    }

    Ok(hex::encode(v1).as_str())
}


/**
 * Reverses an array.
 * @example
 * reverseArray('abcd') = 'cdba'
 */
pub fn reverseArray<T>(arr: &[T]) -> Result<&[T], Error> {
    let sz = arr.len();
    if sz == 0 { Err(()) }

    let mut result = [sz; T];

    for i in 0..sz {
        result[i] = &arr[sz - 1 - i];
    }
    Ok(&result)
}

/**
 * Reverses a HEX string, treating 2 chars as a byte.
 * @example
 * reverseHex('abcdef') = 'efcdab'
 */
pub fn reverseHex(hex: &str) -> Result<&str, Error> {
    let mut out = "";

    for i in (0..hex.len() - 2).rev().step_by(2) {
        out += &hex[i..i + 2];
    }

    OK(out)
}
