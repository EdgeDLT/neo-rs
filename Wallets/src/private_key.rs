use std::{fmt, fmt::Display, str::FromStr};

use failure::Fail;
use serde::{Deserialize, Serialize};

use neo_core::neo_type::{PRIVATE_KEY_BIN_LEN, PRIVATE_KEY_HEX_LEN, WIF_KEY_HEX_LEN, PrivateKeyBin};
use neo_core::no_std::*;
use neo_core::utilities::*;
use neo_crypto::{base58, hex, FromBase58, ToBase58};

use crate::address::{Address, AddressError};
use crate::public_key::PublicKey;
use crate::wif::{WifKey, WifKeyError};
use neo_core::crypto::checksum;
use crate::key_trait::KeyTrait;
use std::error::Error;

/// Represents an  private key
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize)]
pub struct PrivateKey(pub [u8; PRIVATE_KEY_BIN_LEN]);

impl PrivateKey {
    /// Returns a randomly-generated  private key.
    pub fn new<R: Rng>(rng: &mut R) -> Result<Self, PrivateKeyError> {
        let random: [u8; 32] = rng.gen();

        Ok(Self(ran.as_slice()))
    }

    /// Returns the public key of the corresponding  private key.
    pub fn to_public_key(&self) -> Self::PublicKey {
        PublicKey::from_private_key(self)
    }

    /// Returns the address of the corresponding  private key.
    pub fn to_address(&self) -> Result<Address, AddressError> {
        Address::from_private_key(self)
    }

    /// Returns the hex string of the private key.
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex()
    }

    pub fn to_slice(&self) -> PrivateKeyBin {}
    // /// Convert to the wif key
    // pub fn to_wif(&self) -> Result<WifKey, WifKeyError> {}
    //
    // // pub fn from_wif()->Result<Self, PrivateKeyError> {
    // //
    // // }
}

impl KeyTrait for PrivateKey {
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
        let mut key = [0u8; PRIVATE_KEY_BIN_LEN];
        key.copy_from_slice(&self.0);
        key

    }

    fn equals(&self, other: &PrivateKey) -> bool {
        unimplemented!()
    }
}

impl FromStr for PrivateKey {
    type Err = PrivateKeyError;

    fn from_str(private_key: &str) -> Result<Self, PrivateKeyError> {
        match private_key.len() {
            PRIVATE_KEY_HEX_LEN => {
                let secret_key = hex::decode(private_key)?;
                Ok(Self(secret_key.as_slice()))
            }
            WIF_KEY_HEX_LEN => {
                let data = private_key.from_base58().unwrap();
                let len = data.len();

                if (len != 37 && len != 38) ||
                    data[0] != 0x80 ||
                    data[33] != 0x01 {
                    return Err(PrivateKeyError::InvalidByteLength(37));
                };

                let expected = &data[len - 4..len];
                let checksum = &checksum(&data[0..len - 4])[0..4];
                if *expected != *checksum {
                    let expected = expected.to_base58();
                    let found = checksum.to_base58();
                    // println!("Error: {}==>{}", expected, found);
                    return Err(PrivateKeyError::InvalidChecksum(expected, found));
                }

                let mut pk = [0u8; PRIVATE_KEY_BIN_LEN];
                pk.copy_from_slice(&data[1..33]);

                Ok(PrivateKey(pk))
            }

            _ => Err(PrivateKeyError::InvalidCharacterLength(private_key.len()))
        }
        // if private_key.len() != PRIVATE_KEY_HEX_LEN  && private_key.len() != WIF_KEY_HEX_LEN{
        //     return Err(PrivateKeyError::InvalidCharacterLength(private_key.len()));
        // }
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&self.0.serialize());
        write!(f, "{}", hex::encode(private_key).to_string())
    }
}


#[derive(Debug, Fail)]
pub enum PrivateKeyError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "invalid private key checksum: {{ expected: {:?}, found: {:?} }}", _0, _1)]
    InvalidChecksum(String, String),

    #[fail(display = "invalid network: {{ expected: {:?}, found: {:?} }}", _0, _1)]
    InvalidNetwork(String, String),

    #[fail(display = "invalid private key prefix: {:?}", _0)]
    InvalidPrefix(Vec<u8>),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "unsupported format")]
    UnsupportedFormat,
}

impl From<neo_core::no_std::io::Error> for PrivateKeyError {
    fn from(error: neo_core::no_std::io::Error) -> Self {
        PrivateKeyError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<&'static str> for PrivateKeyError {
    fn from(msg: &'static str) -> Self {
        PrivateKeyError::Message(msg.into())
    }
}

impl From<base58::FromBase58Error> for PrivateKeyError {
    fn from(error: base58::FromBase58Error) -> Self {
        PrivateKeyError::Crate("base58", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for PrivateKeyError {
    fn from(error: hex::FromHexError) -> Self {
        PrivateKeyError::Crate("hex", format!("{:?}", error))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_to_public_key(expected_public_key: &PublicKey, private_key: &PrivateKey) {
        let public_key = private_key.to_public_key();
        assert_eq!(*expected_public_key, public_key);
    }

    fn test_to_address(expected_address: &Address, private_key: &PrivateKey) {
        let address = private_key.to_address().unwrap();
        assert_eq!(*expected_address, address);
    }

    // fn test_from_str(
    //     expected_secret_key: &secp256k1::SecretKey,
    //     expected_public_key: &str,
    //     expected_address: &str,
    //     private_key: &str,
    // ) {
    //     let private_key = PrivateKey::from_str(private_key).unwrap();
    //     assert_eq!(*expected_secret_key, private_key.0);
    //     assert_eq!(expected_public_key, private_key.to_public_key().to_string());
    //     assert_eq!(
    //         expected_address,
    //         private_key.to_address().unwrap().to_string()
    //     );
    // }

    fn test_to_str(expected_private_key: &str, private_key: &PrivateKey) {
        assert_eq!(expected_private_key, private_key.to_string());
    }

    mod checksum_address {
        use super::*;

        const KEYPAIRS: [(&str, &str, &str); 5] = [
            (
                "8279d7c0ae2c3266b557845d50ede43e22a7e60408b7c90ee279b8848dbac771",
                "9e984180d8e431b31f51d605639d6eaa447a36189834c10238203aff6c100090738d6a8d293cbc3461d0c17b2ee966364076e37c2ce186acfa6b44d426ac079c",
                "0xA069665F5E31B932b7F5E50FF552A261a694b1DB"
            ),
            (
                "444d0c9a7cb33240a0799a0edc0d89a96b20abf10f91b33d7f5812b49d4f0d95",
                "c86d6b2d319e8267a5dac084aed74c28754b9ea18291ed36d5f1dcf7f9debaef2b25a48d2ae89add88c9797f6f5553235a13db23deac3c8597d52593c056aac3",
                "0xdeA0f51325b69323f0C73e2f81A0a389d55Bbca5"
            ),
            (
                "40d4098958b22c19e866f0761f5d589fcc088b78f4e881bfda7ebee7df044bdd",
                "d1b1ab9c694894950da166520af3081c1f169c7306f2ed8ce507928832aa0429b35476084efd325439f2016f174b3e0243df7f40f92111aaa191c82dd94bf8d7",
                "0x36D0E703Aa4733AFB3CDFC000D66BE65d14fFfc8"
            ),
            (
                "f56ebd9b96ddbd8faf320ae8af2b49aeff4b54dc8867a6c39092fe1aa7434b7e",
                "8d270aba1ed09d353d7c8c892593b628499eb1d714fbaabd9938e43cbb847cefa0435b29f1541ab397b1482c028f95b83f56603f5183f432ae862bcbccf13e04",
                "0x337b22d054eed94C6c0711B3b0bd7DDaE23e5DC5"
            ),
            (
                "ab95d2466269a48e96f92fe36dfcecf67b4a6f9394de9ec7314dd584426a638c",
                "8269368cad7ce74a530954da01db01e4e62f17625869ad10eabf3a261b5ab6d396b0e1e307455d2ae0f63032b748f909fcea2fbaf36a76536cb298ce343d882c",
                "0x020D80b9B932eE57eFDD2eD35cb4d409554013ba"
            )
        ];

        #[test]
        fn to_public_key() {
            KEYPAIRS.iter().for_each(|(private_key, public_key, _)| {
                let public_key = PublicKey::from_str(public_key).unwrap();
                let private_key = PrivateKey::from_str(&private_key).unwrap();
                test_to_public_key(&public_key, &private_key);
            });
        }

        #[test]
        fn to_address() {
            KEYPAIRS.iter().for_each(|(private_key, _, address)| {
                let address = Address::from_str(address).unwrap();
                let private_key = PrivateKey::from_str(&private_key).unwrap();
                test_to_address(&address, &private_key);
            });
        }

        #[test]
        fn from_secp256k1_secret_key() {
            KEYPAIRS
                .iter()
                .for_each(|(expected_private_key, expected_public_key, expected_address)| {
                    let private_key = PrivateKey::from_str(&expected_private_key).unwrap();
                    test_from_secp256k1_secret_key(
                        expected_private_key,
                        expected_public_key,
                        expected_address,
                        private_key.0,
                    );
                });
        }

        #[test]
        fn from_str() {
            KEYPAIRS
                .iter()
                .for_each(|(private_key, expected_public_key, expected_address)| {
                    let expected_private_key = PrivateKey::from_str(&private_key).unwrap();
                    test_from_str(
                        &expected_private_key.0,
                        expected_public_key,
                        expected_address,
                        &private_key,
                    );
                });
        }

        #[test]
        fn to_str() {
            KEYPAIRS.iter().for_each(|(expected_private_key, _, _)| {
                let private_key = PrivateKey::from_str(expected_private_key).unwrap();
                test_to_str(expected_private_key, &private_key);
            });
        }
    }

    #[test]
    fn test_checksum_address_invalid() {
        // Invalid private key length

        let private_key = "8";
        assert!(PrivateKey::from_str(private_key).is_err());

        let private_key = "8279d7c0ae2c3266b557845d50ede43";
        assert!(PrivateKey::from_str(private_key).is_err());

        let private_key = "8279d7c0ae2c3266b557845d50ede43e22a7e60408b7c90ee279b8848dbac77";
        assert!(PrivateKey::from_str(private_key).is_err());

        let private_key =
            "8279d7c0ae2c3266b557845d50ede43e22a7e60408b7c90ee279b8848dbac7718279d7c0ae2c3266b557845d50ede43";
        assert!(PrivateKey::from_str(private_key).is_err());

        let private_key = "8279d7c0ae2c3266b557845d50ede43e22a7e60408b7c90ee279b8848dbac7718279d7c0ae2c3266b557845d50ede43e22a7e60408b7c90ee279b8848dbac771";
        assert!(PrivateKey::from_str(private_key).is_err());
    }
}
