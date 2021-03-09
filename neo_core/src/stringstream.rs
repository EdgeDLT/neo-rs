use crate::misc::reverse_hex;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct StringStream {
    pub s: &'static str,
    pub pter: usize,
}

/**
 * A simple string stream that allows user to read a string byte by byte using read().
 * @param str - The string to read as a stream.
 */
impl StringStream {
    /**
     * Initializes the stream with given string and pointer at position 0.
     */
    pub fn new(str: &'static str) -> StringStream {
        Self {
            s: str.clone(),
            pter: 0,
        }
    }

    /**
     * Checks if reached the end of the stream. Does not mean stream is actually empty (self.s is not empty).
     * @example
     * let ss = new StringStream("01020304");
     * ss.is_empty(); // false
     * ss.pter = 3;
     * ss.is_empty(); // true
     */
    pub fn is_empty(&self) -> bool {
        self.pter >= self.s.len()
    }

    /**
     * Peek at the next bytes on the string. May return less than intended bytes if reaching end of stream.
     * @example
     * let ss = new StringStream("0102");
     * ss.peek();  // "01"
     * ss.peek(5); // "0102"
     */
    pub fn peek(&self, bytes: u32) -> String {
        if self.is_empty() {
            ()
        }

        let from = self.pter as usize;
        let to = (self.pter + (bytes as usize) * 2) as usize;


        String::from(&self.s[from..to])
    }

    /**
     * Reads some bytes off the stream.
     * @param bytes Number of bytes to read
     * @example
     * let ss = new StringStream("01020304");
     * ss.read(); // "01"
     * ss.read(2); // "0203"
     */
    pub fn read(&mut self, bytes: usize) -> String {
        if self.is_empty() {
            ()
        };

        let to = (self.pter + bytes * 2) as usize;
        let out = &self.s[self.pter..to];

        self.pter = self.pter + bytes * 2;

        String::from(out)
    }

    /**
     * Reads some bytes off the stream.
     * A variable-length integer is first read off the stream and then bytes equal to the integer is read off and returned.
     */
    pub fn read_var_bytes(&mut self) -> String {
        let len = self.read_var_int() as usize;
        self.read(len)
    }

    /**
     * Reads an integer of variable bytelength. May consume up to 9 bytes.
     * The first byte read indicates if more bytes need to be read off.
     */
    pub fn read_var_int(&mut self) -> i32 {

        let mut len = i32::from_str_radix(self.read(1).as_str(), 16).unwrap();

        match len {
            0xfd => len = i32::from_str_radix(reverse_hex(self.read(2).as_str()).as_str(), 16).unwrap(),
            0xfe => len = i32::from_str_radix(reverse_hex(self.read(4).as_str()).as_str(), 16).unwrap(),
            0xff => len = i32::from_str_radix(reverse_hex(self.read(8).as_str()).as_str(), 16).unwrap(),
            _ => unreachable!()
        }
        len
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
            true => {
                let from = self.pter - 10;
                &self.s[from..self.pter]
            }

            false => { &self.s[0..self.pter] }
        };

        let current = self.read(1);
        let after = self.peek(5);
        self.pter = self.pter - 2;

        println!("$before => {} | $current => {:?}| $after => {:?}", before, current, after);
    }
}

#[cfg(test)]
mod tests {
    use crate::stringstream::StringStream;

    const HX: &str = "fd2c2b414e815dd71004ffc93367ec53fe0617439d927dbf9aab771354c50f568f8ebd8585a7c88ba99ca2aa6e1aa7d830c7cf546fddfdf49dee2de7bd52fb6bac";

    #[test]
    pub fn test_create_new_ss() {
        let ss = StringStream::new(HX);
        assert_eq!(ss.s, HX);
    }

    #[test]
    pub fn test_read_int() {
        let mut ss = StringStream::new(HX);
         let num = ss.read_var_int();
        assert_eq!(num, 11052)
    }

    #[test]
    pub fn test_read_var_bytes() {
        let temp = "fd04004e815dd71004ff";
         let mut ss = StringStream::new(temp);
        let bytes = ss.read_var_bytes();

        assert_eq!(bytes, "4e815dd7");
    }

    #[test]
    pub fn test_read(){
         let temp = "fd04004e815dd71004ff";
         let mut ss = StringStream::new(temp);
        let bytes = ss.read(5);

        assert_eq!(bytes, "fd04004e81");
    }
}

