use openssl::{
    bn::{BigNum, BigNumContext},
    error::ErrorStack,
};

use neo_core::{neo_type, SignatureHex, PublicKeyHex, KeyPair};
use neo_crypto::{hex, sha2};
use std::io::Error;
use crate::nep2;

use neo_crypto::sha2::Digest;
use neo_core::neo_type::{SignatureHex, PublicKeyHex};
use neo_crypto::ecdsa::{ECECDSA, CipherSuite, Ecdsa};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct signing {}

impl signing {
    /**
     * Converts signatureHex to a signature object with r & s.
     */
    fn get_signature_from_hex(&self, signatureHex: SignatureHex) -> (BigNum, BigNum) {
        let signatureBuffer = hex::decode(signatureHex)?;

        let r = BigNum::from_slice(&signatureBuffer[0..32])?;
        // new BN(signatureBuffer.slice(0, 32).to & str("hex"), 16, "be");
        let s = BigNum::from_slice(&signatureBuffer[32..signatureBuffer.len()])?;

        (r, s)
    }

    /**
     * Generates a ECDSA signature from a hex&str using the given private key.
     * @param hex Hex&str to hash.
     * @param privateKey Hex&str or WIF format.
     */
    pub fn sign(&self, hex: &str, private_key: &str) -> Result<SignatureHex, Error> {
        let mut pri_key = Vec::new();
        match isWIF(private_key) {
            true => pri_key = KeyPair::KeyPair::get_private_key_from_wif(private_key)?.to_vec(),
            false => pri_key = hex::decode(private_key)?,
        }

        let msg = hex::decode(hex)?;
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        Ok(hex::encode(ecdsa.prove(&pri_key, &msg).unwrap()).as_str())
    }

    /**
     * Verifies that the message, signature and signing key matches.
     * @param hex Message that was signed.
     * @param sig ECDSA signature.
     * @param publicKey encoded/unencoded public key of the signing key.
     */
    pub fn verify(&self, hex: &str, sig: SignatureHex, publicKey: PublicKeyHex) -> bool {
        let pubkey = hex::decode(publicKey).unwrap();
        let msg = hex::decode(&hex)?;
        let pi = hex::decode(&sig)?;
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        let beta = ecdsa.verify(&pubkey, &pi, &msg);

        match beta {
            Ok(beta) => {
                println!("VRF proof is valid!\nHash output: {}", hex::encode(&beta));
                true
            }
            Err(e) => {
                false
            }
        }
    }
}