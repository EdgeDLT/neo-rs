// import Fixed8 from "./Fixed8";
// import { ensureHex, reverseHex } from "./misc";
// use hex as hex_str;

use crate::misc::reverseHex;
use crate::fixed8::fixed8;
use neo_crypto::hex;
use std::io::Error;
use std::convert::TryInto;
use std::str;
use std::num::ParseIntError;
use std::iter::FromIterator;


/**
 * @param buf ArrayBuffer
 * @returns ASCII string
 */
pub fn ab2str(buf: &[u8]) -> String {

    String::from(str::from_utf8(buf).unwrap())
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
    hex::encode(arr)
}

/**
 * @param str ASCII string
 * @returns HEX string
 */
pub fn str2hex(s: &str) -> String {

    ab2hexstring(str2ab(s))
}

/**
 * @param hexstring HEX string
 * @returns ASCII string
 */
pub fn hexstring2str(hexstring: &str) -> String {

    let h = hex::decode(hexstring).unwrap();
    let v = str::from_utf8(h.as_slice()).unwrap();

    String::from(v)
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
pub fn num2hexstring(num: i64) -> String {
    format!("{:02X}", num)
}

/**
 * Converts a number to a Fixed8 format hex string
 * @param  num
 * @param size output size in bytes
 * @return number in Fixed8 representation.
 */
pub fn num2fixed8(num: i64) -> fixed8 {
    fixed8(num)
}


/**
 * Converts a number to a variable length Int. Used for array length header
 * @param num
 * @returns hexstring of int.
 */
pub fn num2VarInt(num: i64) -> String {

    match num {
        d if d < 0xfd => num2hexstring(num),
        d if d <= 0xffff => format!("fd{}", num2hexstring(num)),
        d if d <= 0xffffffff => format!("fe{}",num2hexstring(num)),
        _ => format!("ff{}",num2hexstring(num)),
    }
}