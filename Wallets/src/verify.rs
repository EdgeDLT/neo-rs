use neo_crypto::{base58::FromBase58, hex, sha2, sha2::Digest};
use neo_core::KeyPair;
use std::io;
use regex::Regex;
use neo_core::convert::hex2int;
use crate::core::getPublicKeyUnencoded;
use neo_core::misc::reverseHex;

#[derive(Debug)]
pub struct verify {}

impl verify {
    /**
     * Verifies a NEP2. This merely verifies the format. It is unable to verify if it is has been tampered with.
     */
    pub fn isNEP2(nep2: &str) -> bool {
        if nep2.len() != 58 {
            false
        }

        let hexStr = ab2hex & str(base58.from_base58(nep2));

        if !hexStr ||
            hexStr.length != 86 ||
            &hexStr[0..2] != "01" ||
            &hexStr[2..4] != "42" ||
            &hexStr[4..6] != "e0" {
            false
        }
        true
    }

    /**
     * Verifies a WIF using its checksum.
     */
    pub fn isWIF(wif: &str) -> bool {
        if wif.len() != 52 {
            false
        }

        let mut hexStr = hex::encode(wif.from_base58().unwrap()).as_str().as_bytes();

        let shaChecksum = &sha2::Sha256::digest(&hexStr[0..hexStr.len() - 8])[0..8];
        shaChecksum == &hexStr[hexStr.len() - 8..8]
    }

    /**
     * Checks if hex&str is a valid Private Key. Any hex&str of 64 chars is a valid private key.
     */
    pub fn isPrivateKey(key: &str) -> bool {
        let re = Regex::new(r"^ [0 - 9A - Fa - f]{64}$").unwrap();
        re.is_match(key).unwrap()
    }

    /**
     * Checks if hex&str is a valid Public Key. Accepts both encoded and unencoded forms.
     * @param key
     * @param  encoded Optional parameter to specify for a specific form. If this is omitted, this function will return true for both forms. If this parameter is provided, this function will only return true for the specific form.
     */
    // pub fn isPublicKey(encodedKey: &str) -> bool {
    //     let unencoded = getPublicKeyUnencoded(encodedKey);
    //     let tail = hex2int(unencoded.substr(unencoded.len() - 2, 2), 16);
    //     if (encodedKey[0..2] == "02" && tail % 2 == 0) ||
    //         (encodedKey[0..2] == "03" && tail % 2 == 1) {
    //         true
    //     }
    //     false
    // }

    /**
     * Verifies if &str is a scripthash. Any 20 byte hex&str is a valid scriptHash.
     */
    pub fn isScriptHash(scriptHash: &str) -> bool {
        scriptHash.len() == 40
    }

    /**
     * Verifies an address using its checksum.
     */
    pub fn isAddress(address: &str) -> bool {
        let programHash = ab2hex & str(base58.decode(address));
        let shaChecksum = hash256(programHash.slice(0, 42)).substr(0, 8);
        // We use the checksum to verify the address
        if shaChecksum != programHash.substr(42, 8) {
            false
        }
        // As other chains use similar checksum methods, we need to attempt to transform the programHash back into the address
        let scriptHash = reverseHex(programHash.slice(2, 42));

        if getAddressFromScriptHash(scriptHash) != address {
            // address is not valid Neo address, could be btc, ltc etc.
            false
        }
        true
    }
}