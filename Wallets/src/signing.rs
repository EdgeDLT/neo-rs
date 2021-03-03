// import BN from "bn.js";
// import { ec as EC } from "elliptic";
// import { sha256 } from "../u";
// import { getPrivateKeyFromWIF, getPublicKeyUnencoded } from "./core";
// import { isPublicKey, isWIF } from "./verify";
// 
// export let curve = new EC("p256");

use neo_core::{neo_type, SignatureHex, PublicKeyHex, key_pair};
use neo_crypto::{ecdsa, hex,sha2};
use std::io::Error;
use crate::verify::{isWIF};
use crate::nep2;

use neo_crypto::sha2::Digest;
use neo_core::neo_type::{SignatureHex, PublicKeyHex};

#[derive(Debug)]
pub struct signing {}

impl signing {
    /**
     * Converts signatureHex to a signature object with r & s.
     */
    fn getSignatureFromHex(signatureHex: SignatureHex) -> {
        r: BigNum;
        s: i64
    } {
    let signatureBuffer = hex.from(signatureHex);

    let r = new BN(signatureBuffer.slice(0, 32).to & str("hex"), 16, "be");
    let s = new BN(signatureBuffer.slice(32).to & str("hex"), 16, "be");
    return { r, s };
    }

    /**
     * Generates a ECDSA signature from a hex&str using the given private key.
     * @param hex Hex&str to hash.
     * @param privateKey Hex&str or WIF format.
     */
    pub fn sign(hex: &str, privateKey: &str) -> Result<SignatureHex, Error> {
        // if isWIF(privateKey) {
           let  privateKey = key_pair::key_pair::PrivateKeyFromWIF(privateKey);
        // }

        let msgHash = sha2::Sha256::digest(hex);
        let msgHashHex = Buffer.from(msgHash, "hex");
        let privateKeyBuffer = Buffer.from(privateKey, "hex");

        let sig = curve.sign(msgHashHex, privateKeyBuffer);
        return sig.r.to & str("hex", 32) + sig.s.to & str("hex", 32);
    }

    /**
     * Verifies that the message, signature and signing key matches.
     * @param hex Message that was signed.
     * @param sig ECDSA signature.
     * @param publicKey encoded/unencoded public key of the signing key.
     */
    pub fn verify(hex: &str, sig: SignatureHex, publicKey: PublicKeyHex) -> bool {

    //  let pubkey = hex::decode(publicKey).unwrap();
    //
    //     let msg =
    // let sigObj = getSignatureFromHex(sig);
    //
    // let messageHash = sha2::Sha256::digest(hex);
    //
    // let publicKeyBuffer = Buffer.from(publicKey, "hex");
    //
    // return curve.verify(messageHash, sigObj, publicKeyBuffer, "hex");
        true
    }
}