use std::io::Error;
use std::num::ParseIntError;
use std::str;

use neo_crypto::hex;
use neo_crypto::hex::ToHex;

use crate::fixed8::Fixed8;

/**
 * @param buf ArrayBuffer
 * @returns ASCII string
 */
pub fn ab2str(buf: &[u8]) -> String {
    str::from_utf8(buf).unwrap().to_string()
}

/**
 * @param str ASCII string
 * @returns
 */
pub fn str2ab(s: &str) -> &[u8] {
    s.as_bytes()
}

/**
 * @param str HEX string
 * @returns
 */
pub fn hexstring2ab(s: &str) -> Result<Box<[u8]>, Error> {
    Ok(hex::decode(s).unwrap().into_boxed_slice())
}

/**
 * @param arr
 * @returns HEX string
 */
pub fn ab2hexstring(arr: &[u8]) -> String {
    // arr.to_hex()
    hex::encode(arr)
}

/**
 * @param str ASCII string
 * @returns HEX string
 */
pub fn str2hex(s: &str) -> String {
    s.encode_hex()
    // ab2hexstring(str2ab(s))
}

/**
 * @param hexstring HEX string
 * @returns ASCII string
 */
pub fn hex2str(hex_str: &str) -> String {
    let h = hex::decode(hex_str).unwrap();
    let v = str::from_utf8(h.as_slice()).unwrap();
    v.to_string()
}

/**
 * convert an integer to big endian hex and add leading zeros
 * @param num Integer.
 */
pub fn int2hex(num: i32) -> String {
    // num.to_string()
    format!("{:02X}", num)
}

/**
 * Converts a Fixed8 hex string to its original number
 * @param fixed8hex number in Fixed8 representation
 */
pub fn hex2int(hex: &str) -> Result<i64, ParseIntError> {
    i64::from_str_radix(hex, 16)
}

/**
 * Converts a number to a big endian hexstring of a suitable size, optionally little endian
 * @param num A positive integer.
 * @param size The required size in bytes, eg 1 for Uint8, 2 for Uint16. Defaults to 1.
 * @param littleEndian Encode the hex in little endian form
 */
pub fn num2hexstring(num: i64, size: usize) -> String {
    format!("{:01$x}", num, size)
}

/**
 * Converts a number to a Fixed8 format hex string
 * @param  num
 * @param size output size in bytes
 * @return number in Fixed8 representation.
 */
pub fn num2fixed8(num: i64) -> Fixed8 {
    Fixed8(num)
}


/**
 * Converts a number to a variable length Int. Used for array length header
 * @param num
 * @returns hexstring of int.
 */
pub fn num2var_int(num: i64) -> String {
    match num {
        d if d < 0xfd => num2hexstring(num, 1*2),
        d if d <= 0xffff => format!("fd{}", num2hexstring(num, 2*2)),
        d if d <= 0xffffffff => format!("fe{}", num2hexstring(num, 4*2)),
        _ => format!("ff{}", num2hexstring(num, 8*2)),
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::{ab2str, hex2int, int2hex, num2hexstring, str2ab};

    #[test]
    pub fn test_ab2str() {
        let v: Vec<u8> = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64]; //helloworld
        let s = ab2str(&v);
        assert_eq!(s, "hello world");
    }

    #[test]
    pub fn test_str2ab() {
        let s = "hello world";
        let v = str2ab(s);
        assert_eq!(v, [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64]);
    }

    #[test]
    pub fn test_num2hexstring() {
        let i = 92233720;

        let h = num2hexstring(i, 2);
        assert_ne!(h, "F8".to_lowercase());

        let h = num2hexstring(i, 4);
        assert_ne!(h, "5FF8".to_lowercase());

        let h = num2hexstring(i, 8);
        assert_eq!(h, "057F5FF8".to_lowercase());

        let h = num2hexstring(i, 10);
        assert_eq!(h, "00057F5FF8".to_lowercase());

        let h = num2hexstring(i, 12);
        assert_eq!(h, "0000057F5FF8".to_lowercase())
    }

    #[test]
    pub fn test_int2hex() {
        let i = 92233720;
        let h = int2hex(i).to_lowercase();

        assert_eq!(h, "57F5FF8".to_lowercase())
    }

    #[test]
    pub fn test_hex2int() {
        let h = "57F5FF8";
        let i = hex2int(h).unwrap();

        assert_eq!(i, 92233720)
    }
}