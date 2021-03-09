use std::{fmt, fmt::Display, str::FromStr};

use neo_core::neo_type::PUBLIC_KEY_BIN_LEN;
use neo_crypto::{base58, hex};
use secp256k1;
use wagyu_model::{Address, AddressError, PublicKey, PublicKeyError};

use crate::address::{Address, AddressError};
use crate::format::Format;
use crate::private_key::PrivateKey;
use neo_crypto::ecdsa::{ECECDSA, CipherSuite};

/// Represents an  public key
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize, FromStr, Send, Sync, Sized)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_BIN_LEN]);

impl PublicKey {
    /// Returns the address corresponding to the given public key.
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut pub_key = ecdsa.derive_public_key(&sec_key.as_slice()).unwrap();

        Self(pub_key.as_slice())
    }

    /// Returns the address of the corresponding private key.
    pub fn to_address(&self) -> Result<Self::Address, AddressError> {
        Address::from_public_key(self)
    }

    /// Returns a public key given a secp256k1 public key.
    pub fn from_secp256k1_public_key(public_key: secp256k1::PublicKey) -> Self {
        Self(public_key)
    }

    /// Returns the secp256k1 public key of the public key
    pub fn to_secp256k1_public_key(&self) -> secp256k1::PublicKey {
        self.0.clone()
    }
}

impl FromStr for PublicKey {
    type Err = PublicKeyError;

    fn from_str(public_key: &str) -> Result<Self, Self::Err> {
        Ok(Self(secp256k1::PublicKey::parse_slice(
            hex::decode(format!("04{}", public_key).as_str())?.as_slice(), None,
        )?))
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in &self.0.serialize()[1..] {
            write!(f, "{:02x}", s)?;
        }
        Ok(())
    }
}


#[derive(Debug, Fail)]
pub enum PublicKeyError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "invalid public key prefix: {:?}", _0)]
    InvalidPrefix(String),

    #[fail(display = "no public spending key found")]
    NoSpendingKey,

    #[fail(display = "no public viewing key found")]
    NoViewingKey,
}

impl From<crate::no_std::io::Error> for PublicKeyError {
    fn from(error: crate::no_std::io::Error) -> Self {
        PublicKeyError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<base58::FromBase58Error> for PublicKeyError {
    fn from(error: base58::FromBase58Error) -> Self {
        PublicKeyError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::Error> for PublicKeyError {
    fn from(error: bech32::Error) -> Self {
        PublicKeyError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for PublicKeyError {
    fn from(error: hex::FromHexError) -> Self {
        PublicKeyError::Crate("hex", format!("{:?}", error))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_from_private_key(expected_public_key: &PublicKey, private_key: &PrivateKey) {
        let public_key = PublicKey::from_private_key(private_key);
        assert_eq!(*expected_public_key, public_key);
    }

    fn test_to_address(expected_address: &Address, public_key: &PublicKey) {
        let address = public_key.to_address().unwrap();
        assert_eq!(*expected_address, address);
    }

    fn test_from_str(expected_public_key: &str, expected_address: &str) {
        let public_key = PublicKey::from_str(expected_public_key).unwrap();
        let address = public_key.to_address().unwrap();
        assert_eq!(expected_public_key, public_key.to_string());
        assert_eq!(expected_address, address.to_string());
    }

    fn test_to_str(expected_public_key: &str, public_key: &PublicKey) {
        assert_eq!(expected_public_key, public_key.to_string());
    }

    mod checksum_address {
        use super::*;

        const KEYPAIRS: [(&str, &str, &str); 5] = [
            (
                "2f46188bd601ece2a4446fa31de9419ee9baabf5305d65a5a7aea8badee27a5a",
                "06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b0142409760451d982c0f35931f33e57adfc4f11bdf1946be2d75d6ecc925e8d22f319c71a721c",
                "0x9Ed0C5817aE96Cb886BF74EB02B238De682e9B07"
            ),
            (
                "d96c4c30bbabde58653e4fb4f4d97d064c70e300a37ab8780a8ecc15220423fb",
                "bfe0746c85802c3ca1c2d5e4f4d23fb8321b8b1009af67855cc9a4aed8285567d7045bb700e27d5e33572ae5d84a8d1e11bb134f6f14f37ffcb2fa73f7c6b0ac",
                "0xBc90633A78dA594ace8e25AAA3517F924C76099d"
            ),
            (
                "c677a1215eebd35d20337d8896ee6579c78f41f93946b17c8d4ccb772c25cde4",
                "ff3e50efb509efd0d18ff9074bc8b253419d2437e0c1e81661c1ba419f877162eed685d80bdd3b33adde4ff2a0946dd97460f126992064059a129e2a7172d566",
                "0xA99E404A60ab8561F7c844529F735A88D7A61C5A"
            ),
            (
                "b681e5bd4ddffefe1a691fe7c6375775c11992b9a25e4f9e3f235eb054d49343",
                "d9ed72afa68a9732df005df2dbbfb2abcad050579bd8dfeb32389d0f1e492d130ca33f9e71345d558da5859026fee86c03be685f95a4c8ddc55e048c5ff8b398",
                "0x28826C9f713c96ee63e59Ed9220c77b021FAfC3e"
            ),
            (
                "da5d359af6827e76e0a1b71c75c375f0d33f63bae4fd551d81ee10faa34e33e9",
                "0b752d5e89126b62a99edfe40a4cbd9122cfb04257a28d225858d38bc92a0e1517e797e9029e810b329afa32a1d46268e84eb10c700314b0059f506130d1e9e6",
                "0x9eC59170674DbEfeF40efE2ED03175b39fCA921a"
            )
        ];

        #[test]
        fn from_private_key() {
            KEYPAIRS.iter().for_each(|(private_key, public_key, _)| {
                let public_key = PublicKey::from_str(public_key).unwrap();
                let private_key = PrivateKey::from_str(&private_key).unwrap();
                test_from_private_key(&public_key, &private_key);
            });
        }

        #[test]
        fn to_address() {
            KEYPAIRS.iter().for_each(|(_, public_key, address)| {
                let address = Address::from_str(address).unwrap();
                let public_key = PublicKey::from_str(&public_key).unwrap();
                test_to_address(&address, &public_key);
            });
        }

        #[test]
        fn from_str() {
            KEYPAIRS.iter().for_each(|(_, expected_public_key, expected_address)| {
                test_from_str(expected_public_key, expected_address);
            });
        }

        #[test]
        fn to_str() {
            KEYPAIRS.iter().for_each(|(_, expected_public_key, _)| {
                let public_key = PublicKey::from_str(expected_public_key).unwrap();
                test_to_str(expected_public_key, &public_key);
            });
        }
    }

    #[test]
    fn test_checksum_address_invalid() {
        // Invalid public key length

        let public_key = "0";
        assert!(PublicKey::from_str(public_key).is_err());

        let public_key = "06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b014";
        assert!(PublicKey::from_str(public_key).is_err());

        let public_key = "06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b0142409760451d982c0f35931f33e57adfc4f11bdf1946be2d75d6ecc925e8d22f319c71a721";
        assert!(PublicKey::from_str(public_key).is_err());

        let public_key = "06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b0142409760451d982c0f35931f33e57adfc4f11bdf1946be2d75d6ecc925e8d22f319c71a721c06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b0142409760451d982c0f3593";
        assert!(PublicKey::from_str(public_key).is_err());

        let public_key = "06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b0142409760451d982c0f35931f33e57adfc4f11bdf1946be2d75d6ecc925e8d22f319c71a721c06d68e391c6961fceb5d8c5ad8ee5c6346db24df9dae61c9c0b0142409760451d982c0f35931f33e57adfc4f11bdf1946be2d75d6ecc925e8d22f319c71a721c";
        assert!(PublicKey::from_str(public_key).is_err());
    }
}
