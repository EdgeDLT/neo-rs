use crate::no_std::*;
use rand::Rng;
use std::{fmt, fmt::Display, str::FromStr, convert::{TryInto,TryFrom}};
use scrypt::{scrypt, Params};

use crate::utilities::crypto::{checksum, hash160};
use neo_crypto::{ecdsa::{CipherSuite, ecdsa, ECECDSA},
                 base58::{FromBase58, ToBase58},
                 hex,
                 aes::{aes,
                       cipher::{generic_array::GenericArray,
                                {BlockCipher, NewBlockCipher}},
                 }};

use crate::neo_type;
use crate::key_pair;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct nep2 {
}


impl nep2 {


    pub fn GetNep2FromKeyPair() {}


    pub fn GetNep2FromPrivateKey(pri_key: &'static str, passphrase: &'static str) -> String {
        let private_key = pri_key.as_bytes();
        let key_pair = key_pair::key_pair::GetKeyPairFromPrivateKey(&private_key);
        let mut addresshash:[u8;4] = key_pair.GetAddrHashFromAddress();

        let mut result = vec![0u8; neo_type::SCRYPT_DK_LEN];
        let params = Params::new(neo_type::SCRYPT_LOG_N, neo_type::SCRYPT_R, neo_type::SCRYPT_P).unwrap();

        scrypt(
            passphrase.as_bytes(),
            addresshash.to_vec().as_slice(),
            &params,
            &mut result,
        ).unwrap();

        let half_1 = &result[0..32];
        let half_2 = &result[32..64];
        let mut u8xor = [0u8; 32];

        for i in 0..32 {
            u8xor[i] = &private_key[i] ^ half_1[i];
        }

        let cipher = aes::from_key(<[u8; 32]>::try_from(half_2).unwrap());
        let encrypted = cipher.encrypt(Vec::from(u8xor));

        // # Assemble the final result
        let mut assembled = Vec::new();

        assembled.push(neo_type::NEP_HEADER_1);
        assembled.push(neo_type::NEP_HEADER_2);
        assembled.push(neo_type::NEP_FLAG);
        assembled.extend(addresshash.to_vec());
        assembled.extend(encrypted);

        // # Finally, encode with Base58Check
        assembled.to_base58()
    }


    pub fn GetPrivateKeyFromNep2(nep2: &'static str, passphrase: &'static str) -> String{

        if nep2.len() != 58{
            println!("Wrong nep2");
            ()
        }
        let decoded_key:[u8; 39] = nep2.from_base58().unwrap().try_into().unwrap();
        // ADDRESS_HASH_SIZE = 4
        // ADDRESS_HASH_OFFSET = 3

        let mut address_hash: &[u8] = &decoded_key[3..7];
        let encrypted:&[u8] = &decoded_key[7..39];

        // pwd_normalized = bytes(unicodedata.normalize('NFC', passphrase), 'utf-8')
        let mut result = vec![0u8; neo_type::SCRYPT_DK_LEN];
        let params = Params::new(neo_type::SCRYPT_LOG_N, neo_type::SCRYPT_R, neo_type::SCRYPT_P).unwrap();

        scrypt(
            passphrase.as_bytes(),
            &address_hash,
            &params,
            &mut result,
        ).unwrap();


        // derived = scrypt.hash(pwd_normalized, address_hash,
        //                       N=SCRYPT_ITERATIONS,
        //                       r=SCRYPT_BLOCKSIZE,
        //                       p=SCRYPT_PARALLEL_FACTOR,
        //                       buflen=SCRYPT_KEY_LEN_BYTES)

        let half_1 = &result[0..32];
        let half_2 = &result[32..64];

        // derived1 = derived[:32]
        // derived2 = derived[32:]

          let cipher = aes::from_key(<[u8; 32]>::try_from(half_2).unwrap());
        let decrypted = cipher.encrypt(Vec::from(encrypted));

         let mut pri_key = [0u8; 32];

        for i in 0..32 {
            pri_key[i] = decrypted[i] ^ half_1[i];
        }
        // cipher = aes.new(derived2, aes.MODE_ECB)
        // decrypted = cipher.decrypt(encrypted)
        // private_key = xor_bytes(decrypted, derived1)

        let key_pair = key_pair::key_pair::GetKeyPairFromPrivateKey(pri_key.as_ref());
        let mut kp_addresshash:[u8;4] = key_pair.GetAddrHashFromAddress();

        // # Now check that the address hashes match. If they don't, the password was wrong.
        // kp_new = key_pair(priv_key=private_key)
        // kp_new_address = kp_new.GetAddress()
        // kp_new_address_hash_tmp = hashlib.sha256(kp_new_address.encode("utf-8")).digest()
        // kp_new_address_hash_tmp2 = hashlib.sha256(kp_new_address_hash_tmp).digest()
        // kp_new_address_hash = kp_new_address_hash_tmp2[:4]
        if kp_addresshash != address_hash{
            println!("Wrong Passphrase");
        }
        hex::encode(pri_key)
    }
}