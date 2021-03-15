// import util from "util";
// import { DEFAULT_ACCOUNT_CONTRACT, DEFAULT_SCRYPT } from "../consts";
// import logger from "../logging";
// import { hash160, reverseHex } from "../u";
// import * as core from "./core";
// import { construct_multi_sig_verification_script } from "./multisig";
// import { decrypt, encrypt, ScryptParams } from "./nep2";
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

use crate::address::Address;
use crate::private_key::PrivateKey;
use crate::multisig::construct_multi_sig_verification_script;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Contract {
    script: String,
    parameters: Vec<HashMap<String,String>>,
    deployed: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Account {
    address: Address,
    label: &'static str,
    is_default: bool,
    lock: bool,
    key: PrivateKey,
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
     * @param signingThreshold Minimum number of signatures required for verification. Must be larger than 0 and less than number of keys provided.
     * @param publicKeys List of public keys to form the account. 2-16 keys allowed. Order is important.
     * @example
     * let threshold = 2;
     * let publicKeys = [
     * "02028a99826edc0c97d18e22b6932373d908d323aa7f92656a77ec26e8861699ef",
     * "031d8e1630ce640966967bc6d95223d21f44304133003140c3b52004dc981349c9",
     * "02232ce8d2e2063dce0451131851d47421bfc4fc1da4db116fca5302c0756462fa"
     * ];
     * let acct = Account.createMultiSig(threshold, publicKeys);
     */
    pub fn createMultiSig(
        signingThreshold: usize,
        publicKeys: &[str]) -> Account {
        let verificationScript = construct_multi_sig_verification_script(
            signingThreshold,
            publicKeys,
        );
        Account {
            contract: Contract{
                script: verificationScript,
                parameters:  Array(signingThreshold).map((_, i) => ({
                    name: `signature $ { i }`,
                    type : "Signature",
                })),
                deployed: false,
            },
        }
    }

    // public extra: { [key: string]: any };
    // public is_default: boolean;
    // public lock: boolean;

    // public contract: {
    //   script: string;
    //   parameters: { name: string; type: string }[];
    //   deployed: boolean;
    // };
    // public label: string;

    // tslint:disable:variable-name
    // private _privateKey?: string;
    // private _encrypted?: string;
    // private _address?: string;
    // private _publicKey?: string;
    // private _scriptHash?: string;
    // private _WIF?: string;
    // tslint:enables:variable-name

    // public constructor(str: string | Partial<AccountJSON> = "") {
    // self.extra = {};
    // self.label = "";
    // self.is_default = false;
    // self.lock = false;
    // self.contract = Object.assign({}, DEFAULT_ACCOUNT_CONTRACT);
    // if ( ! str) {
    // self._privateKey = core.generatePrivateKey();
    // } else if (typeof str == = "object") {
    // self._encrypted = str.key;
    // self._address = str.address;
    // self.label = str.label | | "";
    // self.extra = str.extra | | {};
    // self.is_default = str.is_default | | false;
    // self.lock = str.lock || false;
    // self.contract =
    // str.contract | | Object.assign({}, DEFAULT_ACCOUNT_CONTRACT);
    // } else if (isPrivateKey(str)) {
    // self._privateKey = str;
    // } else if (isPublicKey(str, false)) {
    // self._publicKey = core.getPublicKeyEncoded(str);
    // } else if (isPublicKey(str, true)) {
    // self._publicKey = str;
    // } else if (isScriptHash(str)) {
    // self._scriptHash = str;
    // } else if (isAddress(str)) {
    // self._address = str;
    // } else if (isWIF(str)) {
    // self._privateKey = core.getPrivateKeyFromWIF(str);
    // self._WIF = str;
    // } else if (isNEP2(str)) {
    // self._encrypted = str;
    // } else {
    // throw new ReferenceError(`Invalid input: ${str}`);
    // }
    //
    // self._update_contract_script();
    // // Attempts to make address the default label of the Account.
    // if ( ! self.label) {
    // try {
    // self.label = self.address;
    // } catch (err) {
    // self.label = "";
    // }
    // }
    // }

    pub fn get_symbol() -> &'static str {
        "Account"
    }

    // public [inspect]() {
    // return `[Account: ${self.label}]`;
    // }

    pub fn is_multi_sig(&self) -> bool {
        self.contract.script &&
            self.contract.script.slice(self.contract.script.length - 2) == "ae"
    }

    /**
     * Key encrypted according to NEP2 standard.
     * @example 6PYLHmDf6AjF4AsVtosmxHuPYeuyJL3SLuw7J1U8i7HxKAnYNsp61HYRfF
     */
    pub fn encrypted(&self) -> String {
        if self._encrypted {
            return self._encrypted;
        } else {
            panic!("No encrypted key found");
        }
    }

    /**
     * Case sensitive key of 52 characters long.
     * @example L1QqQJnpBwbsPGAuutuzPTac8piqvbR1HRjrY5qHup48TBCBFe4g
     */
    pub fn get_wif(&self) -> String {
        if self._WIF {
            return self._WIF;
        } else {
            self._WIF = core.getWIFFromPrivateKey(self.privateKey);
            return self._WIF;
        }
    }

    /**
     * Key of 64 hex characters.
     * @example 7d128a6d096f0c14c3a25a2b0c41cf79661bfcb4a8cc95aaaea28bde4d732344
     */
    pub fn get_private_key(&self) -> String {
        if self._privateKey {
            return self._privateKey;
        } else if self._WIF {
            self._privateKey = core.getPrivateKeyFromWIF(self._WIF);
            return self._privateKey;
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
        if self._publicKey {
            return self._publicKey;
        } else {
            self._publicKey = core.getPublicKeyFromPrivateKey(self.privateKey);
            return self._publicKey;
        }
    }

    /** Retrieves the Public Key in encoded / unencoded form.
     * @param encoded Encoded or unencoded.
     */
    pub fn get_PublicKey(&self, encoded: bool) -> String {
        return encoded
            ?;
        self.publicKey
            : core.getPublicKeyUnencoded(self.publicKey);
    }

    /**
     * Script hash of the key. self format is usually used in the code instead of address as self is a hexstring.
     */
    pub fn get_script_hash(&self) -> String {
        if self._scriptHash {
            return self._scriptHash;
        } else {
            if self._address {
                self._scriptHash = core.getScriptHashFromAddress(self.address);
                return self._scriptHash;
            } else if self.contract.script {
                self._scriptHash = self._get_script_hash_from_verification_script();
                return self._scriptHash;
            } else {
                self._scriptHash = core.getScriptHashFromPublicKey(self.publicKey);
                return self._scriptHash;
            }
        }
    }

    /**
     * Public address used to receive transactions. Case sensitive.
     * @example ALq7AWrhAueN6mJNqk6FHJjnsEoPRytLdW
     */
    pub fn get_address(&self) -> String {
        if self._address {
            return self._address;
        } else {
            self._address = core.getAddressFromScriptHash(self.scriptHash);
            return self._address;
        }
    }

    /**
     * self is the safe way to get a key without it throwing an error.
     */
    pub fn tryGet(
        keyType:
    | "WIF"
    | "privateKey"
    | "publicKey"
    | "encrypted"
    | "scriptHash"
    | "address"
    ) -> String {
    switch (keyType) {
    case "encrypted":
    return self._encrypted | | "";
    case "WIF":
    return self._WIF | | "";
    case "privateKey":
    return self._privateKey || "";
    case "publicKey":
    return self._publicKey | | "";
    case "scriptHash":
    return self._scriptHash | | "";
    case "address":
    return self._address | | "";
    }

    /**
     * Encrypts the current privateKey and return the Account object.
     */
    pub fn encrypt(&self,
                   keyphrase: string,
                   scryptParams: ScryptParams = DEFAULT_SCRYPT,
    ) -> &self {
        return Promise.resolve()
            .then((;_) => encrypt(self.privateKey, keyphrase, scryptParams))
        .then((encrypted) => {
            self._encrypted = encrypted;
            return self;
        });
    }

    /**
     * Decrypts the encrypted key and return the Account object.
     */
    pub fn decrypt(
        &self,
        keyphrase: string,
        scryptParams: ScryptParams = DEFAULT_SCRYPT,
    ) -> &self {
        return Promise.resolve()
            .then((;_) => decrypt(self.encrypted, keyphrase, scryptParams))
        .then((wif) => {
            self._WIF = wif;
            self._update_contract_script();
            return self;
        });
    }

    /**
     * Export Account as a WalletAccount object.
     */
    pub fn export(&self) -> Account {
        let mut key = "";
        if self._privateKey && !self._encrypted {
            panic!("Encrypt private key first!");
        }
        if self._encrypted {
            key = self._encrypted;
        }
        Account {
            address: self.address,
            label: self.label,
            is_default: self.is_default,
            lock: self.lock,
            key,
            contract: self.contract.clone(),
            extra: self.extra,
        }
    }

    pub fn equals(&self, other: &Account) -> bool {
        self.address == other.address
    }

    /**
     * Attempts to update the contract.script field if public key is available.
     */
    fn _update_contract_script(&self) {
        if self.contract.script == "" {
            let publicKey = self.publicKey;
            self.contract.script = core.getVerificationScriptFromPublicKey(
                publicKey
            );
            self._scriptHash = self._get_script_hash_from_verification_script();
        }
    }

    fn _get_script_hash_from_verification_script(&self) -> String {
        return reverseHex(hash160(self.contract.script));
    }
}

