use crate::no_std::*;
use rand::Rng;
use std::{fmt, fmt::Display, str::FromStr, convert::TryInto};
use crate::utilities::crypto::{checksum, hash160};
use neo_crypto::{ecdsa::{CipherSuite, ECECDSA},
                 base58::{FromBase58, ToBase58},
                 hex
};

use crate::{neo_type};
use std::convert::TryFrom;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct key_pair {
    pub public_key: neo_type::PublicKeyBin,
    pub private_key: neo_type::PrivateKeyBin,
    pub address: neo_type::AddressBin,
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
            public_key: <[u8; neo_type::PUBLIC_KEY_BIN_LEN]>::try_from(pub_key).unwrap(),
            address: addr,
        }
    }


    pub fn GetKeyPairFromPrivateKey(pri_key:&[u8]) -> key_pair{

         let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // let mut rng = rand::rngs::OsRng {};
        // let sec_key= pri_key;
        let pub_key= ecdsa.derive_public_key(&pri_key).unwrap();

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

        Self {
            private_key: <[u8; neo_type::PRIVATE_KEY_BIN_LEN]>::try_from(pri_key).unwrap(),
            public_key: <[u8; neo_type::PUBLIC_KEY_BIN_LEN]>::try_from(pub_key).unwrap(),
            address: addr,
        }

    }
    pub fn GetPublicKey(&self) -> &neo_type::PublicKeyBin {

        &self.public_key
    }

    pub fn GetAddress(&self) -> &neo_type::AddressBin {

        &self.address
    }

    pub  fn GetAddrHashFromAddress(&self)-> [u8; 4]{

        let cs = &checksum(&self.address)[0..4];
        cs.try_into().unwrap()
        // slice_as_array::slice_as_array!(cs, [u8; 4]).expect("bad hash length")
        // .try_into().expect("slice with incorrect length");
    }

    pub fn GetPublicKeyFromPrivateKey(pri_key: Vec<u8>) -> [u8; neo_type::PUBLIC_KEY_BIN_LEN] {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        let pk =  ecdsa.derive_public_key(&pri_key).unwrap();

        pk.try_into().unwrap()
        // slice_as_array::slice_as_array!(pk, [u8; neo_type::PUBLIC_KEY_BIN_LEN]).expect("bad hash length")
    }

    pub fn PrivateKeyFromWIF(&self, wif: &str) -> Result<Self, KeyPairError> {
        let data = wif.from_base58()?;
        let len = data.len();
        if len != 37 && len != 38 || data[0] != 0x80 || data[33] != 0x01 {
            return Err(KeyPairError::InvalidByteLength(len));
        }

        let expected = &data[len - 4..len];
        let checksum = &checksum(&data[0..len - 4])[0..4];
        if *expected != *checksum {
            let expected = expected.to_base58();
            let found = checksum.to_base58();
            return Err(KeyPairError::InvalidChecksum(expected, found));
        }

        let mut pk = [0u8; neo_type::PRIVATE_KEY_BIN_LEN];
        pk.copy_from_slice(&data[1..33]);

        Ok(Self {
            private_key: pk,
            public_key: *self.GetPublicKey(),
            ..Default::default()
        })
    }


    pub fn WifFromPrivateKey(&self) -> String {
        let mut wif = [0u8; neo_type::WIF_KEY_BIN_LEN];
        wif[0] = 0x80;
        wif[33] = 0x01;
        wif[1..33].copy_from_slice(&self.private_key);
        let sum = &checksum(&wif[0..34])[0..4];

        {
            wif[34..].copy_from_slice(sum);
            wif.to_base58()
        }
    }

    pub fn PrivateKeyFromKeyPair(&self, KeyPair_key: Vec<u8>, passphrase: Vec<u8>) {}


    pub fn Export(&self) {}
    pub fn ExportKeyPair(&self, passphrase: Vec<u8>) {}

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

impl  Default for key_pair {
    fn default() -> Self { Self { private_key:[0u8; neo_type::PRIVATE_KEY_BIN_LEN], public_key:[0u8; neo_type::PUBLIC_KEY_BIN_LEN], address:[0u8; neo_type::ADDRESS_BIN_LEN]}}
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

impl From<secp256k1::Error> for KeyPairError {
    fn from(error: secp256k1::Error) -> Self {
        KeyPairError::Crate("libsecp256k1", format!("{:?}", error))
    }
}
