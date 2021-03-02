use crate::no_std::*;
use rand::Rng;
use std::{fmt, fmt::Display, str::FromStr, convert::TryInto, io::Error};
use crate::utilities::crypto::{checksum, hash160};
use neo_crypto::{ecdsa::{CipherSuite, ECECDSA},
                 base58::{FromBase58, ToBase58},
                 hex,
};

use crate::{neo_type, PublicKeyBin, PrivateKeyBin, AddressBin, AddressHex};
use std::convert::TryFrom;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct key_pair {
    pub public_key: PublicKeyBin,
    pub private_key: PrivateKeyBin,
    pub address: AddressHex,
}


impl key_pair {
    pub fn new() -> key_pair {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut rng = rand::rngs::OsRng {};
        let sec_key: [u8; neo_type::PRIVATE_KEY_BIN_LEN] = rng.gen();

        let mut pub_key = ecdsa.derive_public_key(&sec_key).unwrap();

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
        Self {
            private_key: sec_key,
            public_key: <PublicKeyBin>::try_from(pub_key).unwrap(),
            address: addr.to_base58().as_str(),
        }
    }


    pub fn get_public_key_from_private_key(&self, pri_key: &PrivateKeyBin) -> Result<&PublicKeyBin, Error> {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        let pk = ecdsa.derive_public_key(&pri_key).unwrap();

        Ok(pk.try_into().unwrap())
    }

    pub fn get_key_pair_from_private_key(&self, pri_key: &PrivateKeyBin) -> key_pair {

        let pub_key = self.get_public_key_from_private_key(pri_key).unwrap();

       let mut addr = self.get_address_from_public_key(pub_key).unwrap().as_str();

        Self {
            private_key: <PrivateKeyBin>::try_from(pri_key).unwrap(),
            public_key: <PublicKeyBin>::try_from(pub_key).unwrap(),
            address: addr,
        }
    }

    pub fn get_public_key(&self) -> Result<&PublicKeyBin, Error> {
        Ok(&self.public_key)
    }

    pub fn get_address(&self) -> Result<&AddressHex, Error> {
        Ok(&self.address)
    }

    pub fn get_addr_hash_from_address(&self) -> [u8; 4] {
        let cs = &checksum(&self.address.from_base58().unwrap())[0..4];
        cs.try_into().unwrap()
    }


    pub fn get_key_pair_from_wif(&self, wif: &str) -> Result<Self, KeyPairError> {
        let pk: PrivateKeyBin = self.get_private_key_from_wif(wif).unwrap();

        Ok(Self {
            private_key: pk,
            public_key: *self.get_public_key().unwrap(),
            ..Default::default()
        })
    }

    pub fn get_private_key_from_wif(wif: &str) -> Result<PrivateKeyBin, Error> {
        let data = wif.from_base58()?;
        let len = data.len();

        if len != 37 && len != 38 || data[0] != 0x80 || data[33] != 0x01 {
            Err(())
        }

        let expected = &data[len - 4..len];
        let checksum = &checksum(&data[0..len - 4])[0..4];
        if *expected != *checksum {
            let expected = expected.to_base58();
            let found = checksum.to_base58();
            println!("Error: {}==>{}", expected, found);
            Err(());
        }

        let mut pk = [0u8; neo_type::PRIVATE_KEY_BIN_LEN];
        pk.copy_from_slice(&data[1..33]);

        Ok(pk)
    }


    pub fn get_wif_from_private_key(&self, pri_key: &PrivateKeyBin) -> Result<&str, Error> {
        let mut wif = [0u8; neo_type::WIF_KEY_BIN_LEN];
        wif[0] = 0x80;
        wif[33] = 0x01;
        wif[1..33].copy_from_slice(pri_key);
        let sum = &checksum(&wif[0..34])[0..4];
        {
            wif[34..].copy_from_slice(sum);
            Ok(wif.to_base58().as_str())
        }
    }

    pub fn get_address_from_public_key(&self, pub_key: &PublicKeyBin) -> Result<String, Error> {
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

        Ok(addr.to_base58())
    }

    pub fn get_private_key_from_key_pair(&self, KeyPair_key: Vec<u8>, passphrase: Vec<u8>) {}


    pub fn export(&self) {}
    pub fn export_key_pair(&self, passphrase: Vec<u8>) {}
}

impl fmt::Display for key_pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let wif = self.WifFromPrivateKey();
        write!(f, "Public Key:\t 0x{}\nPrivate Key:\t 0x{}\nWIF:\t\t {}\nAddress:\t {}",
               &hex::encode(&self.public_key),
               &hex::encode(&self.private_key),
               wif,
               &self.address.to_base58())
    }
}

impl Default for key_pair {
    fn default() -> Self { Self {
        private_key: [0u8; neo_type::PRIVATE_KEY_BIN_LEN],
        public_key: [0u8; neo_type::PUBLIC_KEY_BIN_LEN],
        address: "" } }
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