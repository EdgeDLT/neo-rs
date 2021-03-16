use std::error::Error;
use std::str::FromStr;

use neo_crypto::{FromBase58, ToBase58};

use crate::private_key::PrivateKey;
use crate::key_trait::KeyTrait;

// WIF represents a wallet import format.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct WIF {
    // version of the wallet import format. Default to 0x80.
    version: u8,

    // Bool to determine if the WIF is compressed or not.
    compressed: bool,

    // A reference to the private_key which this WIF is created from.
    private_key: PrivateKey,

    // The string representation of the WIF.
    s: String,
}

impl WIF {
    const WIFVersion: u8 = 0x80;

    // wif encode encodes the given private key into a WIF string.
    pub fn wif_encode(key: &[u8], mut version: u8, compressed: bool) -> Result<String, dyn Error> {
        if version == 0x00 {
            version = WIFVersion
        }

        if key.len() != 32 {
            return Err(WifKeyError::InvalidByteLength(key.len()));
        }

        let mut buf = Vec::new();
        buf.push(version);
        buf.extend_from_slice(key);

        if compressed {
            buf.push(0x01);
        }

        Ok(buf.to_base58())
    }

    // wif decode decodes the given WIF string into a WIF struct.
    pub fn wif_decode(wif: &str, mut version: u8) -> Result<Self, dyn Error> {
        let mut b = wif.from_base58().unwrap();

        if version == 0x00 {
            version = WIFVersion
        }

        if b[0] != version {
            return Err(Error::new(ErrorKind::Invalid, "invalid WIF version got {}, expected {}", b[0], version));
        }

        let mut w = WIF {
            version,
            compressed: false,
            private_key: PrivateKey::from_str(wif).unwrap(),
            s: wif.to_string(),
        };

        // This is an uncompressed WIF.
        if b.len() == 33 {
            w.compressed = false;
            return Ok(w);
        }

        if b.len() != 34 {
            return Err(WifKeyError::InvalidByteLength(b.len()));
        }

        // Check the compression flag.
        if b[33] != 0x01 {
            return Err(WifKeyError::UnsupportedFormat);
        }
        w.compressed = true;
        Ok(w)
    }
}


impl KeyTrait for WIF{
    fn deserialize(&self, hex: &str) -> Result<_, dyn Error> {
        unimplemented!()
    }

    fn serialize(&self) -> Result<String, dyn Error> {
        unimplemented!()
    }

    fn to_hex(&self) -> String {
        unimplemented!()
    }

    fn to_slice(&self) -> &[u8] {
        unimplemented!()
    }

    fn equals(&self, other: &_) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
pub enum WifKeyError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "unsupported format")]
    UnsupportedFormat,
}
