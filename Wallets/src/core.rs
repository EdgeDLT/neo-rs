use std::error::Error;
use neo_core::convert::{hexstring2ab, ab2hexstring};
use neo_core::misc::reverse_hex;
use neo_crypto::{FromBase58, hex, sha2};
use neo_core::consts::ADDR_VERSION;
use neo_core::crypto::hash160;
use neo_crypto::sha2::Digest;

/**
 * @file Core methods for manipulating keys
 * NEP2 <=> WIF <=> Private => Public => ScriptHash <=> Address
 * Keys are arranged in order of derivation.
 * Arrows determine the direction.
 *
 * NEP2 methods are found within NEP2 standard.
 * All methods take in Big-Endian strings and return Big-Endian strings.
 */


/**
 * Encodes a public key.
 * @param unencodedKey unencoded public key
 * @return encoded public key
 */
pub fn getPublicKeyEncoded(unencodedKey: string) -> Result<&str, dyn Error> {
    let mut publicKeyArray = hexstring2ab(unencodedKey)?;

    match publicKeyArray[64] % 2 {
        1 => Ok("03" + ab2hexstring(&publicKeyArray[1..33])),
        0 => Ok("02" + ab2hexstring(&publicKeyArray[1..33])),
        _ => Err(())
    }
}

/**
 * Unencodes a public key.
 * @param  publicKey Encoded public key
 * @return decoded public key
 */
pub fn getPublicKeyUnencoded(publicKey: string) -> Result<&str, dyn Error> {
    let publicKeyBuffer = Buffer.from(publicKey, "hex");
    let keyPair = curve.keyFromPublic(publicKeyBuffer, "hex");
    return keyPair.getPublic().encode("hex", false);
}

/**
 * Converts a private key to WIF.
 */
pub fn getWIFFromPrivateKey(privateKey: string) -> Result<&str,dyn Error> {
    return WIF.encode(128, Buffer.from(privateKey, "hex"), true);
}


/**
 * Converts a public key to verification script form.
 * VerificationScript serves a very niche purpose.
 * It is attached as part of the signature when signing a Transaction.
 * Thus, the name 'scriptHash' instead of 'keyHash' is because we are hashing the verificationScript and not the PublicKey.
 */
pub fn get_verification_script_from_public_key(
    public_key: &str
) -> Result<&str, dyn Error> {

    Ok("21" + public_key + "ac")
}

/**
 * Converts a public key to scripthash.
 */
pub fn get_script_hash_from_public_key(public_key: string) -> Result<&str,dyn Error> {

    let verif_script = get_verification_script_from_public_key(public_key)?;
    Ok(hex::encode(hash160(verif_script.as_bytes()).reverse()).as_str())
}

// /**
//  * Converts a scripthash to address.
//  */
// pub fn get_address_from_script_hash(script_hash: &str) -> Result<&str,dyn Error> {
//
//     let mut hash = reverse_hex(script_hash);
//
//     let mut shaChecksum = &sha2::Sha256::digest(ADDR_VERSION + script_hash)[0..8];
//
//     (ADDR_VERSION + script_hash + shaChecksum)
// }

/**
 * Converts an address to scripthash.
 */
pub fn get_script_hash_from_address(addr: &str) -> Result<&str,dyn Error> {
    let mut hash = &addr.from_base58()?[1..21];
    hash.reverse();
    Ok(hex::encode(hash).as_str())
}

/**
 * Generates a signature of the Transaction based on given private key.
 * @param tx Serialized unsigned Transaction.
 * @param privateKey Private Key.
 * @return Signature. Does not include tx.
 */
pub fn generate_signature(tx: &str, privateKey: &str) -> Result<&str,dyn Error> {
     sign(tx, privateKey);
}