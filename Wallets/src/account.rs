// import util from "util";
// import { DEFAULT_ACCOUNT_CONTRACT, DEFAULT_SCRYPT } from "../consts";
// import logger from "../logging";
// import { hash160, reverseHex } from "../u";
// import * as core from "./core";
// import { construct_multi_sig_verification_script } from "./multisig";
// import { decrypt, encrypt, ScryptParams } from "./Nep2";
// import {
//   isAddress,
//   isNEP2,
//   isPrivateKey,
//   isPublicKey,
//   isScriptHash,
//   isWIF,
// } from "./verify";
//
// let log = logger("wallet");

// let inspect = util.inspect.custom;

use std::collections::HashMap;

use neo_core::crypto::hash160;
use neo_core::neo_type::PrivateKeyHex;

use crate::address::Address;
use crate::multisig::construct_multi_sig_verification_script;
use crate::nep2::Nep2;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;
use crate::wif::WIF;
use crate::key_trait::KeyTrait;
use std::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Account {
    pub(crate) _private_key: Option<PrivateKey>,
    pub(crate) _encrypted: Option<String>,
    pub(crate) _address: Option<Address>,
    pub(crate) _public_key: Option<PublicKey>,
    pub(crate) _script_hash: Option<String>,
    pub(crate) _wif: Option<WIF>,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Contract {
    script: &'static str,
    parameters: Vec<HashMap<&'static str, &'static str>>,
    deployed: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccountJSON {
    address: &'static str,
    label: &'static str,
    is_default: bool,
    lock: bool,
    key: &'static str,
    contract: Contract,
    extra: Option<HashMap<String, String>>,
}

/**
 * self allows for simple utilisation and manipulating of keys without need the long access methods.
 *
 * Key formats are derived from each other lazily and stored for future access.
 * If the previous key (one level higher) is not found, it will attempt to generate it or throw an Error if insufficient information was provided (eg. trying to generate private key when only address was given.)
 *
 * NEP2 <=> WIF <=> Private => Public => ScriptHash <=> Address
 *
 * @param str WIF/ Private Key / Public Key / Address or a Wallet Account object.
 * @example
 * let acct = new Account("L1QqQJnpBwbsPGAuutuzPTac8piqvbR1HRjrY5qHup48TBCBFe4g");
 * acct.address; // "ALq7AWrhAueN6mJNqk6FHJjnsEoPRytLdW"
 */
impl Account {
    /**
     * Create a multi-sig account from a list of public keys
     * @param signing_threshold Minimum number of signatures required for verification. Must be larger than 0 and less than number of keys provided.
     * @param public_keys List of public keys to form the account. 2-16 keys allowed. Order is important.
     * @example
     * let threshold = 2;
     * let public_keys = [
     * "02028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
     * "031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
     * "02232ce8d2e2063dce0451131851d47421bfc4fc1da4db116fca5302c0756462fa"
     * ];
     * let acct = Account::create_multi_sig(threshold, public_keys);
     */
    pub fn create_multi_sig(&mut self,
                            signing_threshold: usize,
                            public_keys: &[str]) -> &self {
        let verification_script = construct_multi_sig_verification_script(
            signing_threshold,
            public_keys,
        );

        self.contract = Contract {
            script: verification_script.as_str(),
            parameters: (0..signing_threshold).map(|i| ("signature" + i, "Signature")).collect(),
            deployed: false,
        };

        &self
    }

    pub fn get_symbol() -> &'static str {
        "Account"
    }

    // public [inspect]() {
    // return `[Account: ${self.label}]`;
    // }

    pub fn is_multi_sig(&self) -> bool {
        let l = self.contract.script.len();
        &self.contract.script[l - 2..l] == "ae"
    }

    /**
     * Key encrypted according to NEP2 standard.
     * @example 6PYLHmDf6AjF4AsVtosmxHuPYeuyJL3SLuw7J1U8i7HxKAnYNsp61HYRfF
     */
    pub fn encrypted(&self) -> String {
        if self._encrypted {
            return self._encrypted?;
        } else {
            panic!("No encrypted key found");
        }
    }

    /**
     * Case sensitive key of 52 characters long.
     * @example L1QqQJnpBwbsPGAuutuzPTac8piqvbR1HRjrY5qHup48TBCBFe4g
     */
    pub fn get_wif(&self) -> Option<String> {
        self._wif
    }

    /**
     * Key of 64 hex characters.
     * @example 7d128a6d096f0c14c3a25a2b0c41cf79661bfcb4a8cc95aaaea28bde4d732344
     */
    pub fn get_private_key(&self) -> String {
        if self._private_key {
            return self._private_key?;
        } else if self._wif {
            self._private_key = core.getPrivateKeyFromWIF(self._wif);
            return self._private_key?;
        } else if self._encrypted {
            panic!("Private Key encrypted!");
        } else {
            panic!("No Private Key provided!");
        }
    }

    /**
     * Returns the public key in encoded form. self is the form that is the short version (starts with 02 or 03). If you require the unencoded form, do use the publicKey method instead of self getter.
     * @example 02028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef
     */
    pub fn get_public_key(&self) -> String {
        match self._public_key {
            Some(p) => p,
            None => {
                self._public_key = core.getPublicKeyFromPrivateKey(self.key.to_slice());
                self._public_key
            }
        }
    }

    /**
     * Script hash of the key. self format is usually used in the code instead of address as self is a hexstring.
     */
    pub fn get_script_hash(&self) -> String {
        return if self._scriptHash {
            self._scriptHash
        } else {
            if self._address {
                self._scriptHash = core.getScriptHashFromAddress(self.address);
                self._scriptHash
            } else if self.contract.script {
                self._scriptHash = self._get_script_hash_from_verification_script();
                self._scriptHash
            } else {
                self._scriptHash = core.getScriptHashFromPublicKey(self.publicKey);
                self._scriptHash
            }
        };
    }

    /**
     * Public address used to receive transactions. Case sensitive.
     * @example ALq7AWrhAueN6mJNqk6FHJjnsEoPRytLdW
     */
    pub fn get_address(self) -> String {
        match self._address {
            Some(addr) => addr,
            None => {
                self._address = core.getAddressFromScriptHash(self.scriptHash);
                self._address?
            }
        }
    }

    /**
     * self is the safe way to get a key without it throwing an error.
     */
    pub fn tryGet(&self, key_type: &str) -> &Option<String> {
        match key_type.to_lowercase().as_str() {
            "encrypted" => &self._encrypted,
            "WIF" => &self._wif,
            "privateKey" =>
                &self._private_key,
            "publicKey" =>
                &self._public_key,
            "scriptHash" =>
                &self._scriptHash,
            "address" =>
                &self._address,
            _ => None
        }
    }

    /**
     * Encrypts the current privateKey and return the Account object.
     */
    pub fn encrypt(mut self, keyphrase: &str) -> Self {
        let encrypted = Nep2::get_nep2_from_private_key(&(&self.key.to_hex_string() as PrivateKeyHex), keyphrase).unwrap();
        self._encrypted = Some(encrypted.to_string());

        self
    }

    /**
     * Decrypts the encrypted key and return the Account object.
     */
    pub fn decrypt(mut self, keyphrase: &str) -> Self {
        let decrypted = Nep2::get_private_key_from_nep2(&self._encrypted?, keyphrase).unwrap();
        self._private_key = Some(decrypted.to_string());

        // self
        // return Promise.resolve()
        //     .then((;_) => decrypt(self.encrypted, keyphrase, scryptParams))
        // .then((wif) => {
        //     self._wif = wif;
        //     self._update_contract_script();
        //     return self;
        // });

        self
    }

    /**
     * Export Account as a WalletAccount object.
     */
    pub fn export(&self) -> AccountJSON {
        let mut key = "";
        if self._private_key && !self._encrypted {
            panic!("Encrypt private key first!");
        }
        if self._encrypted {
            key = self._encrypted;
        }
        AccountJSON {
            address: self._address?.clone(),
            label: self.label,
            is_default: self.is_default,
            lock: self.lock,
            key,
            contract: self.contract.clone(),
            extra: &self.extra,
        };

        self
    }

    pub fn equals(&self, other: &Account) -> bool {
        self.address == other.address
    }



    /**
     * Attempts to update the contract.script field if public key is available.
     */
    fn _update_contract_script(mut self) -> Self {
        if self.contract.script == "" {
            let public_key = self.publicKey;
            self.contract.script = core.getVerificationScriptFromPublicKey(
                public_key
            );
            self._scriptHash = self._get_script_hash_from_verification_script();
        }
        self
    }

    fn _get_script_hash_from_verification_script(&self) -> String {
        return reverseHex(hash160(self.contract.script));
    }
}

impl KeyTrait for Account{
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
        unimplemented!()
    }

    fn equals(&self, other: &_) -> bool {
        unimplemented!()
    }
}

