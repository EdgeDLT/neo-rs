use neo_crypto::{base58::FromBase58, hex, sha2, sha2::Digest};
use neo_core::KeyPair;
use std::io;
use regex::Regex;
use neo_core::convert::{hex2int, ab2hexstring};
use crate::core::getPublicKeyUnencoded;
use neo_core::misc::reverse_hex;
use openssl::sha::sha256;

#[derive(Debug)]
pub struct verify {}

impl verify {
    /**
     * Verifies a NEP2. This merely verifies the format. It is unable to verify if it is has been tampered with.
     */
    pub fn is_nep2(nep: &str) -> bool {

        if nep.len() != 58 {
            return false;
        }

        let hex_str =  ab2hexstring(nep.from_base58().unwrap().as_slice());

        if (!hex_str) || (hex_str.len() != 86) ||
            &hex_str[0..2] != "01" ||
            &hex_str[2..4] != "42" ||
            &hex_str[4..6] != "e0" {
            return false;
        }
        true
    }

    /**
     * Verifies a WIF using its checksum.
     */
    pub fn isWIF(wif: &str) -> bool {
        if wif.len() != 52 {
            return false;
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
    pub fn isAddress(addr: &str) -> bool {
        let programHash = ab2hexstring(base58.decode(addr).unwrap());
        let shaChecksum = sha256::digest(programHash.slice(0, 42)).substr(0, 8);
        // We use the checksum to verify the address
        if shaChecksum != programHash.substr(42, 8) {
            false
        }
        // As other chains use similar checksum methods, we need to attempt to transform the programHash back into the address
        let scriptHash = reverse_hex(&programHash[2..42]);

        if getAddressFromScriptHash(scriptHash) != address {
            // address is not valid Neo address, could be btc, ltc etc.
            false
        }
        true
    }
}