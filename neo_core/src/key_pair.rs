use crate::no_std::*;
use rand::Rng;
use std::{fmt, fmt::Display, str::FromStr, convert::TryInto, io::Error};
use crate::utilities::crypto::{checksum, hash160};
use neo_crypto::{ecdsa::{CipherSuite, ECECDSA},
                 base58::{FromBase58, ToBase58},
                 hex,
};


use std::convert::TryFrom;
use std::borrow::Borrow;
use crate::neo_type::{PublicKeyBin, PrivateKeyBin, AddressHex, PRIVATE_KEY_BIN_LEN, WIF_KEY_BIN_LEN, PUBLIC_KEY_BIN_LEN};
use std::io::ErrorKind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyPair {
    pub public_key: PublicKeyBin,
    pub private_key: PrivateKeyBin,
    pub address: AddressHex,
}


impl KeyPair {
    pub fn new() -> KeyPair {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut rng = rand::thread_rng();
        let mut sec_key = [0u8;32];

         let ran:[u8;32] = rng.gen();

        sec_key.copy_from_slice(ran.as_slice());

        let mut pub_key = ecdsa.derive_public_key(&sec_key.as_slice()).unwrap();

        let mut script: Vec<u8> = Vec::new();
        script.push(33);
        script.extend(&pub_key);
        script.push(172);

        let hs = hash160(&script);

        let mut addr = [0u8; 25];
        addr[0] = 23;
        addr[1..21].copy_from_slice(&hs);

        let sum = &checksum(&addr[0..21])[0..4];

        addr[21..25].copy_from_slice(sum);
        // println!("{}",&pub_key.len());

        // let pubkk = pub_key.clone();
        let mut pubk = [0u8; 33];
        pubk.clone_from_slice(pub_key.as_slice());


        let add = addr.to_base58();

        KeyPair {
            private_key: sec_key.clone(),
            public_key: pubk.clone(),
            address:  String::from(add),
        }
    }


    pub fn get_public_key_from_private_key(pri_key: &[u8]) -> Result<PublicKeyBin, Error> {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        let pk = ecdsa.derive_public_key(&pri_key).unwrap();
        let pkk = pk.as_slice();

        Ok(<[u8; 33]>::try_from(pkk).unwrap())
    }

    pub fn get_KeyPair_from_private_key(pri_key: &[u8]) -> KeyPair {

        let pub_key = KeyPair::get_public_key_from_private_key(&pri_key).unwrap().clone();
        let mut addr = KeyPair::get_address_from_public_key(&pub_key).unwrap();

        Self {
            private_key: <[u8; 32]>::try_from(pri_key).unwrap(),
            public_key:pub_key,
            address: String::from(addr),
        }
    }

    pub fn get_public_key(&self) -> Result<&PublicKeyBin, Error> {
        Ok(&self.public_key)
    }

    pub fn get_address(&self) -> Result<&AddressHex, Error> {
        Ok(&self.address)
    }

    pub fn get_addr_hash(&self) -> [u8; 4] {
        let cs = &checksum(&self.address.from_base58().unwrap())[0..4];
        cs.try_into().unwrap()
    }

    pub fn get_addr_hash_from_address(addr:&str) ->Result<Box<[u8]>, Error> {
        let cs = checksum(&addr.from_base58().unwrap());

        Ok(cs.into_boxed_slice())
    }

    pub fn get_key_pair_from_wif(&self, wif: &str) -> Result<KeyPair, KeyPairError> {

        let pk: PrivateKeyBin = self.get_private_key_from_wif(&wif).unwrap();

        let pubk = self.get_public_key()?;

        Ok(KeyPair {
            private_key: pk.clone(),
            public_key: pubk.clone(),
            ..Default::default()
        })
    }

    pub fn get_private_key_from_wif(&self, wif: &str) -> Result<PrivateKeyBin, Error> {
        let data = wif.from_base58().unwrap();
        let len = data.len();

        if (len != 37 && len != 38) ||
            data[0] != 0x80 ||
            data[33] != 0x01 {

            // Err(Error::new(ErrorKind::Other, "Not a subset"))
            ()
        };

        let expected = &data[len - 4..len];
        let checksum = &checksum(&data[0..len - 4])[0..4];
        if *expected != *checksum {
            let expected = expected.to_base58();
            let found = checksum.to_base58();
            println!("Error: {}==>{}", expected, found);
            // Err(());
            ()
        }

        let mut pk = [0u8; PRIVATE_KEY_BIN_LEN];
        pk.copy_from_slice(&data[1..33]);

        Ok(pk)
    }

    pub fn get_wif_from_private_key(pri_key: &[u8]) -> Result<String, Error> {
        let mut wif = [0u8; WIF_KEY_BIN_LEN];
        wif[0] = 0x80;
        wif[33] = 0x01;
        wif[1..33].copy_from_slice(pri_key);
        let sum = &checksum(&wif[0..34])[0..4];
        {
            wif[34..].copy_from_slice(sum);
            Ok(wif.to_base58())
        }
    }

    pub fn get_address_from_public_key(pub_key: &[u8]) -> Result<String, Error> {
        let mut script: Vec<u8> = Vec::new();
        script.push(33);
        script.extend(pub_key);
        script.push(172);

        let hs = hash160(&script);
        let mut addr = [0u8; 25];
        addr[0] = 23;
        addr[1..21].copy_from_slice(&hs);

        let sum = &checksum(&addr[0..21])[0..4];
        addr[21..25].copy_from_slice(sum);

        Ok(addr.to_base58())
    }

    pub fn get_private_key_from_KeyPair(KeyPair_key: Vec<u8>, passphrase: Vec<u8>) {}


    pub fn export(&self) {}
    pub fn export_KeyPair(&self, passphrase: Vec<u8>) {}
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let wif = KeyPair::get_wif_from_private_key(&self.private_key).unwrap();
        write!(f, "Public Key:\t 0x{}\nPrivate Key:\t 0x{}\nWIF:\t\t {}\nAddress:\t {}",
               &hex::encode(&self.public_key),
               &hex::encode(&self.private_key),
               wif,
               self.address)
    }
}

impl Default for KeyPair {
    fn default() -> Self { Self {
        private_key: [0u8; PRIVATE_KEY_BIN_LEN],
        public_key: [0u8; PUBLIC_KEY_BIN_LEN],
        address: String::from("")
    } }
}

#[derive(Debug, Fail)]
pub enum KeyPairError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(
    display = "invalid checksum: {{ expected: {:?}, found: {:?} }}",
    _0, _1
    )]
    InvalidChecksum(String, String),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "unsupported format")]
    UnsupportedFormat,
}

impl From<crate::no_std::io::Error> for KeyPairError {
    fn from(error: crate::no_std::io::Error) -> Self {
        KeyPairError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<&'static str> for KeyPairError {
    fn from(msg: &'static str) -> Self {
        KeyPairError::Message(msg.into())
    }
}

impl From<neo_crypto::FromBase58Error> for KeyPairError {
    fn from(error: neo_crypto::FromBase58Error) -> Self {
        KeyPairError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::Error> for KeyPairError {
    fn from(error: bech32::Error) -> Self {
        KeyPairError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for KeyPairError {
    fn from(error: hex::FromHexError) -> Self {
        KeyPairError::Crate("hex", format!("{:?}", error))
    }
}

impl From<rand_core::Error> for KeyPairError {
    fn from(error: rand_core::Error) -> Self {
        KeyPairError::Crate("rand", format!("{:?}", error))
    }
}


#[cfg(test)]
mod tests {
    use neo_crypto::ecdsa::{ECECDSA, CipherSuite};
    use rand::Rng;
    use neo_crypto::hex;
    use crate::KeyPair;

    #[test]
    pub fn test_get_pub_key_from_private_key(){
        let private_key = "1d9d6b11b9570e50a8511de539be9d125dda022b7d65452acc03de3aa3e87d6c";
        let pri_key = hex::decode(private_key).unwrap();

        let pub_key = KeyPair::get_public_key_from_private_key(&pri_key).unwrap();

        let pub_key_hex = hex::encode(&pub_key);

        assert_eq!(pub_key_hex,"03f9e9a50af13ccec64feedb45d558815ba6d3a3e8c3a727be7f97bb9eeca80f52")
    }

    #[test]
    pub fn test_get_address_from_public_key(){
        let public_key = "03f9e9a50af13ccec64feedb45d558815ba6d3a3e8c3a727be7f97bb9eeca80f52";

        let pub_key = hex::decode(public_key).unwrap();
        let addr = KeyPair::get_address_from_public_key(&pub_key).unwrap();
        assert_eq!(addr, "AHV5J1bVXAvM3eVDrCXx34U1QQnNKeKX1F");
    }

}