// import { reverseHex } from "./misc";

use std::io::Error;
use crate::misc::reverseHex;

#[derive(Debug,Clone,Hash,Eq, PartialEq)]
pub struct StringStream {
    pub s: &'static str,
    pub pter: u32,
}

/**
 * A simple string stream that allows user to read a string byte by byte using read().
 * @param str - The string to read as a stream.
 */
impl StringStream {
    /**
     * Initializes the stream with given string and pointer at position 0.
     */
    pub fn new(str: &str) -> StringStream {
        Self {
            s: str.clone(),
            pter: 0,
        }
    }

    /**
     * Checks if reached the end of the stream. Does not mean stream is actually empty (self.s is not empty).
     * @example
     * let ss = new StringStream("01020304");
     * ss.isEmpty(); // false
     * ss.pter = 3;
     * ss.isEmpty(); // true
     */
    pub fn isEmpty(&self) -> bool {
        self.pter >= self.s.len() as u32
    }

    /**
     * Peek at the next bytes on the string. May return less than intended bytes if reaching end of stream.
     * @example
     * let ss = new StringStream("0102");
     * ss.peek();  // "01"
     * ss.peek(5); // "0102"
     */
    pub fn peek(&self, bytes: u32) -> Result<String, Error> {
        if self.isEmpty() {
            Err(())
        }

        Ok(self.s[self.pter..self.pter + bytes * 2])
    }

    /**
     * Reads some bytes off the stream.
     * @param bytes Number of bytes to read
     * @example
     * let ss = new StringStream("01020304");
     * ss.read(); // "01"
     * ss.read(2); // "0203"
     */
    pub fn read(&mut self, bytes: u32) -> Result<String, Error> {
        if self.isEmpty() {
            Err(())
        }
        let out = self.s[self.pter..self.pter + bytes * 2];

        self.pter = self.pter + bytes * 2;

        Ok(out)
    }

    /**
     * Reads some bytes off the stream.
     * A variable-length integer is first read off the stream and then bytes equal to the integer is read off and returned.
     */
    pub fn readVarBytes(&mut self) -> Result<String, Error> {
        self.read(self.readVarInt()?)
    }

    /**
     * Reads an integer of variable bytelength. May consume up to 9 bytes.
     * The first byte read indicates if more bytes need to be read off.
     */
    pub fn readVarInt(&mut self) -> Result<u32, Error> {
        let mut len = i32::from_str_radix(self.read(1)?.as_str(), 16)?;

        match len {
            0xfd => len = i32::from_str_radix(reverseHex(self.read(2)?.as_str())?, 16)?,
            0xfe => len = i32::from_str_radix(reverseHex(self.read(4)?.as_str())?, 16)?,
            0xff => len = i32::from_str_radix(reverseHex(self.read(8)?.as_str())?, 16)?,
            _ => unreachable!()
        }
        Ok(len as u32)
    }

    /**
     * Resets the pointer to start of string.
     * @example
     * let ss = new StringStream("010203");
     * ss.read(); //"01"
     * ss.reset();
     * ss.read(); // "01"
     */
    pub fn reset(&mut self) {
        self.pter = 0;
    }

    /**
     * Returns a printable string of the characters around the pointer.
     * Used for debugging.
     */
    pub fn context(&mut self) {
        let before = match self.pter > 10 {
            true => { &self.s[self.pter - 10..self.pter] }
            false => { &self.s[0..self.pter] }
        };

        let current = self.read(1)?;
        let after = self.peek(5)?;
        self.pter = self.pter - 2;

        println!("$before => {} | $current => {:?}| $after => {:?}", before, current, after);
        // return `${before}|${current}|${after}`;
    }
}