use std::{convert::TryFrom, fmt, str::FromStr};

use regex::Regex;

use neo_core::to_hex_string;
use neo_crypto::{base58, hex, ToBase58, FromBase58};
use serde::Serialize;
use tiny_keccak::keccak256;
use wagyu_model::{Address, AddressError, PrivateKey, to_hex_string};

use crate::format::Format;
use crate::private_key::{PrivateKey, PrivateKeyError};
use crate::public_key::{PublicKey, PublicKeyError};
use neo_core::crypto::{hash160, checksum};

/// Represents an  address
#[derive(Debug, FromStr,Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    /// Returns the address corresponding to the given private key.
    pub fn from_private_key(private_key: &PrivateKey) -> Result<Self, AddressError> {
        Self::from_public_key(&private_key.to_public_key())
    }

    /// Returns the address corresponding to the given public key.
    pub fn from_public_key(public_key: &PublicKey) -> Result<Self, AddressError> {
        Ok(Self::checksum_address(public_key))
    }

    /// Returns the checksum address given a public key.
    pub fn checksum_address(public_key: &PublicKey) -> Self {

        let mut script: Vec<u8> = Vec::new();
        script.push(33);
        script.extend(&public_key);
        script.push(172);

        let hs = hash160(&script);

        let mut addr = [0u8; 25];
        addr[0] = 23;
        addr[1..21].copy_from_slice(&hs);

        let sum = &checksum(&addr[0..21])[0..4];

        addr[21..25].copy_from_slice(sum);
        let mut pubk = [0u8; 33];
        pubk.clone_from_slice(pub_key.as_slice());

        Address(addr.to_base58())
    }
}

impl<'a> TryFrom<&'a str> for Address {
    type Error = AddressError;

    fn try_from(address: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(address)
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Ok(Address(address.to_string()))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Fail)]
pub enum AddressError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid format conversion from {:?} to {:?}", _0, _1)]
    IncompatibleFormats(String, String),

    #[fail(display = "invalid address: {}", _0)]
    InvalidAddress(String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "invalid address checksum: {{ expected: {:?}, found: {:?} }}", _0, _1)]
    InvalidChecksum(String, String),

    #[fail(display = "invalid network: {{ expected: {:?}, found: {:?} }}", _0, _1)]
    InvalidNetwork(String, String),

    #[fail(display = "invalid address prefix: {:?}", _0)]
    InvalidPrefix(Vec<u8>),

    #[fail(display = "invalid address prefix length: {:?}", _0)]
    InvalidPrefixLength(usize),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "missing public spend key and/or public view key")]
    MissingPublicKey,

    #[fail(display = "{}", _0)]
    PrivateKeyError(PrivateKeyError),

    #[fail(display = "{}", _0)]
    PublicKeyError(PublicKeyError),
}

impl From<crate::no_std::io::Error> for AddressError {
    fn from(error: crate::no_std::io::Error) -> Self {
        AddressError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<crate::no_std::FromUtf8Error> for AddressError {
    fn from(error: crate::no_std::FromUtf8Error) -> Self {
        AddressError::Crate("crate::no_std", format!("{:?}", error))
    }
}

impl From<&'static str> for AddressError {
    fn from(msg: &'static str) -> Self {
        AddressError::Message(msg.into())
    }
}

impl From<PrivateKeyError> for AddressError {
    fn from(error: PrivateKeyError) -> Self {
        AddressError::PrivateKeyError(error)
    }
}

impl From<PublicKeyError> for AddressError {
    fn from(error: PublicKeyError) -> Self {
        AddressError::PublicKeyError(error)
    }
}

impl From<base58::FromBase58Error> for AddressError {
    fn from(error: base58::FromBase58Error) -> Self {
        AddressError::Crate("base58", format!("{:?}", error))
    }
}

impl From<base58_monero::base58::Error> for AddressError {
    fn from(error: base58_monero::base58::Error) -> Self {
        AddressError::Crate("base58_monero", format!("{:?}", error))
    }
}

impl From<bech32::Error> for AddressError {
    fn from(error: bech32::Error) -> Self {
        AddressError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<core::str::Utf8Error> for AddressError {
    fn from(error: core::str::Utf8Error) -> Self {
        AddressError::Crate("core::str", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for AddressError {
    fn from(error: hex::FromHexError) -> Self {
        AddressError::Crate("hex", format!("{:?}", error))
    }
}

impl From<rand_core::Error> for AddressError {
    fn from(error: rand_core::Error) -> Self {
        AddressError::Crate("rand", format!("{:?}", error))
    }
}


#[cfg(test)]
mod tests {
    use wagyu_model::public_key::PublicKey;

    use super::*;

    fn test_from_private_key(expected_address: &str, private_key: &PrivateKey) {
        let address = Address::from_private_key(private_key).unwrap();
        assert_eq!(expected_address, address.to_string());
    }

    fn test_from_public_key(expected_address: &str, public_key: &PublicKey) {
        let address = Address::from_public_key(public_key).unwrap();
        assert_eq!(expected_address, address.to_string());
    }

    fn test_from_str(expected_address: &str) {
        let address = Address::from_str(expected_address).unwrap();
        assert_eq!(expected_address, address.to_string());
    }

    fn test_to_str(expected_address: &str, address: &Address) {
        assert_eq!(expected_address, address.to_string());
    }

    mod checksum_address {
        use super::*;

        const KEYPAIRS: [(&str, &str); 5] = [
            (
                "f89f23eaeac18252fedf81bb8318d3c111d48c19b0680dcf6e0a8d5136caf287",
                "0x9141B7539E7902872095C408BfA294435e2b8c8a",
            ),
            (
                "a93701ea343247db13466f6448ffbca658726e2b4a77530db3eca3c9250b4f0d",
                "0xa0967B1F698DC497A694FE955666D1dDd398145C",
            ),
            (
                "de61e35e2e5eb9504d52f5042126591d80144d49f74b8ced68f4959a3e8edffd",
                "0xD5d13d1dD277BB9041e560A63ee29c086D370b0A",
            ),
            (
                "56f01d5e01b6fd1cc123d8d1eae0d148e00c025b5be2ef624775f7a1b802e9c1",
                "0xc4488ebbE882fa2aF1D466CB2C8ecafE316c067a",
            ),
            (
                "363af8b4d3ff22bb0e4ffc2ff198b4b5be0316f8a507ad5fe32f021c3d1ae8ad",
                "0xF9001e6AEE6EA439D713fBbF960EbA76f4770E2B",
            ),
        ];

        #[test]
        fn from_private_key() {
            KEYPAIRS.iter().for_each(|(private_key, address)| {
                let private_key = PrivateKey::from_str(private_key).unwrap();
                test_from_private_key(address, &private_key);
            });
        }

        #[test]
        fn from_public_key() {
            KEYPAIRS.iter().for_each(|(private_key, address)| {
                let private_key = PrivateKey::from_str(private_key).unwrap();
                let public_key = PublicKey::from_private_key(&private_key);
                test_from_public_key(address, &public_key);
            });
        }

        #[test]
        fn from_str() {
            KEYPAIRS.iter().for_each(|(_, address)| {
                test_from_str(address);
            });
        }

        #[test]
        fn to_str() {
            KEYPAIRS.iter().for_each(|(_, expected_address)| {
                let address = Address::from_str(expected_address).unwrap();
                test_to_str(expected_address, &address);
            });
        }
    }

    #[test]
    fn test_checksum_address_invalid() {
        // Mismatched keypair

        let private_key = "f89f23eaeac18252fedf81bb8318d3c111d48c19b0680dcf6e0a8d5136caf287";
        let expected_address = "0xF9001e6AEE6EA439D713fBbF960EbA76f4770E2B";

        let private_key = PrivateKey::from_str(private_key).unwrap();
        let address = Address::from_private_key(&private_key).unwrap();
        assert_ne!(expected_address, address.to_string());

        let public_key = PublicKey::from_private_key(&private_key);
        let address = Address::from_public_key(&public_key).unwrap();
        assert_ne!(expected_address, address.to_string());

        // Invalid address length

        let address = "9";
        assert!(Address::from_str(address).is_err());

        let address = "0x9";
        assert!(Address::from_str(address).is_err());

        let address = "0x9141B7539E7902872095C408BfA294435e2b8c8";
        assert!(Address::from_str(address).is_err());

        let address = "0x9141B7539E7902872095C408BfA294435e2b8c8a0x9141B7539E7902872095";
        assert!(Address::from_str(address).is_err());

        let address = "0x9141B7539E7902872095C408BfA294435e2b8c8a0x9141B7539E7902872095C408BfA294435e2b8c8a";
        assert!(Address::from_str(address).is_err());
    }
}


