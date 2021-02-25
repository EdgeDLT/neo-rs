#![no_std]
#![deny(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

pub use cipher::{self, BlockCipher, NewBlockCipher, generic_array::{typenum::UTerm, GenericArray}
};

pub use aes_soft::{Aes128, Aes192, Aes256};

use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};

use block_modes::{block_padding::Pkcs7, BlockMode, Ecb};

use hmac::{Hmac, Mac, NewMac};

use sha3::Sha3_256;

type U8Array<Size> = GenericArray<u8, Size>;

type HmacSha256 = Hmac<Sha3_256>;
type Aes256Ecb = Ecb<Aes256, Pkcs7>;

pub type KeySize = [u8; 32];

pub struct aes {
    aes: Aes256,
    aes_varlen: Aes256Ecb,
    key: KeySize,
}

impl Debug for aes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("aes")
            .field("key", &self.key)
            .finish()
    }
}

impl aes {
    /// Creates a new [`aes`] from a key.
    pub fn from_key(key: KeySize) -> Self {
        let arr = GenericArray::from_slice(&key);
        let blank = U8Array::<UTerm>::default();

        let cipher = Aes256::new(&arr);
        let varlen = Aes256Ecb::new(cipher.clone(), &blank);

        aes {
            aes: cipher,
            aes_varlen: varlen,
            key,
        }
    }

    /// Creates a new [`aes`] from a password.
    pub fn from_pass(password: &[u8]) -> Self {
        let mac = HmacSha256::new_varkey(password).expect("somehow the key was invalid length");
        let password_hash = mac.finalize().into_bytes();

        Self::from_key(password_hash.try_into().unwrap())
    }

    pub fn set_key(&mut self, key: KeySize) {
        let arr = GenericArray::from_slice(&key);

        let blank: cipher::generic_array::GenericArray<
            u8,
            cipher::generic_array::typenum::UTerm,
        > = Default::default();

        let cipher = Aes256::new(&arr);
        let varlen = Aes256Ecb::new(cipher.clone(), &blank);

        self.aes = cipher;
        self.aes_varlen = varlen;
    }

    pub fn get_key(&self) -> KeySize {
        self.key
    }

    pub fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        let cipher = self.aes_varlen.clone();
        cipher.encrypt_vec(&data)
    }

    /// Decrypt some data.
    pub fn decrypt(&self, data: Vec<u8>) -> Vec<u8> {
        let cipher = self.aes_varlen.clone();
        cipher.decrypt_vec(&data).expect("Block mode error?")
    }
}