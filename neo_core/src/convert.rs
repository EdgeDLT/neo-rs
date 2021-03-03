// import Fixed8 from "./Fixed8";
// import { ensureHex, reverseHex } from "./misc";

use crate::misc::reverseHex;
use crate::fixed8::fixed8;
use neo_crypto::hex;
use std::io::Error;
use std::convert::TryInto;
use std::str;
use std::num::ParseIntError;

/**
 * @param buf ArrayBuffer
 * @returns ASCII string
 */
pub fn ab2str(buf: &[u8]) -> &str {
    str::from_utf8(buf).unwrap()
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
pub fn hexstring2ab(s: &str) -> Result<&[u8], Error> {
    hex::decode(s).unwrap().try_into()
}

/**
 * @param arr
 * @returns HEX string
 */
pub fn ab2hexstring(arr: &[u8]) -> &str {
    hex::encode(arr).as_str()
}

/**
 * @param str ASCII string
 * @returns HEX string
 */
pub fn str2hex(s: &str) -> &str {
    ab2hexstring(str2ab(s));
}

/**
 * @param hexstring HEX string
 * @returns ASCII string
 */
pub fn hexstring2str(hexstring: &str) -> &str {
    ab2str(hexstring2ab(hexstring).unwrap());
}

/**
 * convert an integer to big endian hex and add leading zeros
 * @param num Integer.
 */
pub fn int2hex(num: i32) -> &str {
    let h = num.toString(16);
    match h.len() % 2 {
        1 => "0" + h,
        0 => h,
    }
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
pub fn num2hexstring(num: i32) -> &str {
    format!("{:02X}", num).as_str()
}

/**
 * Converts a number to a Fixed8 format hex string
 * @param  num
 * @param size output size in bytes
 * @return number in Fixed8 representation.
 */
pub fn num2fixed8(num: i32) -> fixed8 {
    fixed8(num as i64)
}


/**
 * Converts a number to a variable length Int. Used for array length header
 * @param num
 * @returns hexstring of int.
 */
pub fn num2VarInt(num: i32) -> &str {
    match num {
        d if d < 0xfd => num2hexstring(num),
        d if d <= 0xffff => "fd" + num2hexstring(num),
        d if d <= 0xffffffff => "fe" + num2hexstring(num),
        _ => "ff" + num2hexstring(num),
    }
}