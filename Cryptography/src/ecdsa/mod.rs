use std::{
    fmt::{Debug, Formatter},
    os::raw::c_ulong,
};
use std::fmt;

use failure::Fail;
use hmac_sha256::HMAC;
use openssl::{
    bn::{BigNum, BigNumContext},
    ec::{EcGroup, EcPoint, PointConversionForm},
    error::ErrorStack,
    hash::{hash, MessageDigest},
    nid::Nid,
};


use self::utils::{append_leading_zeros, bits2int, bits2octets};

pub trait Ecdsa<PublicKey, SecretKey> {
    type Error;

    fn prove(&mut self, x: SecretKey, alpha: &[u8]) -> Result<Vec<u8>, Self::Error>;

    fn verify(&mut self, y: PublicKey, pi: &[u8], alpha: &[u8]) -> Result<Vec<u8>, Self::Error>;
}

mod utils;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CipherSuite {
    P256_SHA256_TAI,
    SECP256K1_SHA256_TAI,
    K163_SHA256_TAI,
}

impl CipherSuite {
    fn suite_string(&self) -> u8 {
        match *self {
            CipherSuite::P256_SHA256_TAI => 0x01,
            CipherSuite::SECP256K1_SHA256_TAI => 0xFE,
            CipherSuite::K163_SHA256_TAI => 0xFF,
        }
    }
}

#[derive(Debug, Fail)]
pub enum Error {

    #[fail(display = "Error with code {}", code)]
    CodedError { code: c_ulong },

    #[fail(display = "Hash to point function could not find a valid point")]
    HashToPointError,

    #[fail(display = "The proof length is invalid")]
    InvalidPiLength,

    #[fail(display = "The proof is invalid")]
    InvalidProof,

    #[fail(display = "Unknown error")]
    Unknown,
}

impl From<ErrorStack> for Error {
    fn from(error: ErrorStack) -> Self {
        match error.errors().get(0).map(openssl::error::Error::code) {
            Some(code) => Error::CodedError { code },
            _ => Error::Unknown {},
        }
    }
}

pub struct ECECDSA {
    bn_ctx: BigNumContext,
    cipher_suite: CipherSuite,
    cofactor: u8,
    group: EcGroup,
    hasher: MessageDigest,
    order: BigNum,
    qlen: usize,
    n: usize,
}



impl Debug for ECECDSA {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("ECECDSA")
            .field("cipher_suite", &self.cipher_suite)
            .field("cofactor", &self.cofactor)
            .field("qlen", &self.qlen)
            .field("n", &self.n)
            .field("order", &self.order)
            .finish()
    }
}

impl ECECDSA {

    pub fn from_suite(suite: CipherSuite) -> Result<Self, Error> {
        let mut bn_ctx = BigNumContext::new()?;

        let (group, cofactor) = match suite {
            CipherSuite::P256_SHA256_TAI => {
                (EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?, 0x01)
            }
            CipherSuite::K163_SHA256_TAI => (EcGroup::from_curve_name(Nid::SECT163K1)?, 0x02),
            CipherSuite::SECP256K1_SHA256_TAI => (EcGroup::from_curve_name(Nid::SECP256K1)?, 0x01),
        };

        let mut order = BigNum::new()?;
        group.order(&mut order, &mut bn_ctx)?;
        let mut a = BigNum::new()?;
        let mut b = BigNum::new()?;
        let mut p = BigNum::new()?;
        group.components_gfp(&mut p, &mut a, &mut b, &mut bn_ctx)?;
        let n = ((p.num_bits() + (p.num_bits() % 2)) / 2) as usize;
        let qlen = order.num_bits() as usize;

        let hasher = MessageDigest::sha256();

        Ok(ECECDSA {
            cipher_suite: suite,
            group,
            bn_ctx,
            order,
            hasher,
            n,
            qlen,
            cofactor,
        })
    }

    fn derive_public_key_point(&mut self, secret_key: &BigNum) -> Result<EcPoint, Error> {
        let mut point = EcPoint::new(&self.group.as_ref())?;
        point.mul_generator(&self.group, &secret_key, &self.bn_ctx)?;
        Ok(point)
    }

    pub fn derive_public_key(&mut self, secret_key: &[u8]) -> Result<Vec<u8>, Error> {
        let secret_key_bn = BigNum::from_slice(&secret_key)?;
        let point = self.derive_public_key_point(&secret_key_bn)?;
        let bytes = point.to_bytes(
            &self.group,
            PointConversionForm::COMPRESSED,
            &mut self.bn_ctx,
        )?;
        Ok(bytes)
    }

    fn generate_nonce(&mut self, secret_key: &BigNum, data: &[u8]) -> Result<BigNum, Error> {
        let data_hash = hash(self.hasher, &data)?;

        let data_trunc = bits2octets(&data_hash, self.qlen, &self.order, &mut self.bn_ctx)?;
        let padded_data_trunc = append_leading_zeros(&data_trunc, self.qlen);

        let padded_secret_key_bytes: Vec<u8> =
            append_leading_zeros(&secret_key.to_vec(), self.qlen);

        let mut v = [0x01; 32];
        let mut k = [0x00; 32];

        for prefix in 0..2u8 {
            k = HMAC::mac(
                [
                    &v[..],
                    &[prefix],
                    &padded_secret_key_bytes[..],
                    &padded_data_trunc[..],
                ]
                .concat()
                .as_slice(),
                &k,
            );
            v = HMAC::mac(&v, &k);
        }

        loop {
            v = HMAC::mac(&v, &k);
            let ret_bn = bits2int(&v, self.qlen)?;

            if ret_bn > BigNum::from_u32(0)? && ret_bn < self.order {
                return Ok(ret_bn);
            }
            k = HMAC::mac([&v[..], &[0x00]].concat().as_slice(), &k);
            v = HMAC::mac(&v, &k);
        }
    }

    fn hash_to_try_and_increment(
        &mut self,
        public_key: &EcPoint,
        alpha: &[u8],
    ) -> Result<EcPoint, Error> {
        let mut c = 0..255;
        let pk_bytes = public_key.to_bytes(
            &self.group,
            PointConversionForm::COMPRESSED,
            &mut self.bn_ctx,
        )?;
        let cipher = [self.cipher_suite.suite_string(), 0x01];
        let mut v = [&cipher[..], &pk_bytes[..], &alpha[..], &[0x00]].concat();
        let position = v.len() - 1;
        // `Hash(cipher||PK||data)`
        let mut point = c.find_map(|ctr| {
            v[position] = ctr;
            let attempted_hash = hash(self.hasher, &v);
            // Check validity of `H`
            match attempted_hash {
                Ok(attempted_hash) => self.arbitrary_string_to_point(&attempted_hash).ok(),
                _ => None,
            }
        });

        if let Some(pt) = point.as_mut() {
            let mut new_pt = EcPoint::new(&self.group.as_ref())?;
            new_pt.mul(
                &self.group.as_ref(),
                &pt,
                &BigNum::from_slice(&[self.cofactor])?.as_ref(),
                &self.bn_ctx,
            )?;
            *pt = new_pt;
        }
        point.ok_or(Error::HashToPointError)
    }

    fn arbitrary_string_to_point(&mut self, data: &[u8]) -> Result<EcPoint, Error> {
        let mut v = vec![0x02];
        v.extend(data);
        let point = EcPoint::from_bytes(&self.group, &v, &mut self.bn_ctx)?;
        Ok(point)
    }

    fn hash_points(&mut self, points: &[&EcPoint]) -> Result<BigNum, Error> {
        let point_bytes: Result<Vec<u8>, Error> = points.iter().try_fold(
            vec![self.cipher_suite.suite_string(), 0x02],
            |mut acc, point| {
                let bytes: Vec<u8> = point.to_bytes(
                    &self.group,
                    PointConversionForm::COMPRESSED,
                    &mut self.bn_ctx,
                )?;
                acc.extend(bytes);

                Ok(acc)
            },
        );
        let to_be_hashed = point_bytes?;
        let mut hash = hash(self.hasher, &to_be_hashed).map(|hash| hash.to_vec())?;
        hash.truncate(self.n / 8);
        let result = BigNum::from_slice(hash.as_slice())?;

        Ok(result)
    }

    fn decode_proof(&mut self, pi: &[u8]) -> Result<(EcPoint, BigNum, BigNum), Error> {
        let gamma_oct = if self.qlen % 8 > 0 {
            self.qlen / 8 + 2
        } else {
            self.qlen / 8 + 1
        };
        let c_oct = if self.n % 8 > 0 {
            self.n / 8 + 1
        } else {
            self.n / 8
        };

        if pi.len() * 8 < gamma_oct + c_oct * 3 {
            return Err(Error::InvalidPiLength);
        }
        let gamma_point = EcPoint::from_bytes(&self.group, &pi[0..gamma_oct], &mut self.bn_ctx)?;
        let c = BigNum::from_slice(&pi[gamma_oct..gamma_oct + c_oct])?;
        let s = BigNum::from_slice(&pi[gamma_oct + c_oct..])?;

        Ok((gamma_point, c, s))
    }

    fn gamma_to_hash(&mut self, gamma: &EcPoint) -> Result<Vec<u8>, Error> {
        let mut gamma_cof = EcPoint::new(&self.group.as_ref())?;
        gamma_cof.mul(
            &self.group.as_ref(),
            &gamma,
            &BigNum::from_slice(&[self.cofactor])?.as_ref(),
            &self.bn_ctx,
        )?;

        let gamma_string = gamma_cof.to_bytes(
            &self.group,
            PointConversionForm::COMPRESSED,
            &mut self.bn_ctx,
        )?;

        let hash = hash(
            self.hasher,
            &[
                &[self.cipher_suite.suite_string()],
                &[0x03],
                &gamma_string[..],
            ]
            .concat(),
        )
        .map(|hash| hash.to_vec())?;

        Ok(hash)
    }


    pub fn proof_to_hash(&mut self, pi: &[u8]) -> Result<Vec<u8>, Error> {
        let (gamma_point, _, _) = self.decode_proof(&pi)?;

        self.gamma_to_hash(&gamma_point)
    }
}


impl Ecdsa<&[u8], &[u8]> for ECECDSA {
    type Error = Error;

    fn prove(&mut self, x: &[u8], alpha: &[u8]) -> Result<Vec<u8>, Error> {
        //TODO: validate secret key length?
        let secret_key = BigNum::from_slice(x)?;
        let public_key_point = self.derive_public_key_point(&secret_key)?;

        let h_point = self.hash_to_try_and_increment(&public_key_point, alpha)?;

        let h_string = h_point.to_bytes(
            &self.group,
            PointConversionForm::COMPRESSED,
            &mut self.bn_ctx,
        )?;

        // Step 4: Gamma = x * H
        let mut gamma_point = EcPoint::new(&self.group.as_ref())?;
        gamma_point.mul(&self.group.as_ref(), &h_point, &secret_key, &self.bn_ctx)?;

        // Step 5: nonce
        let k = self.generate_nonce(&secret_key, &h_string)?;

        // Step 6: c = hash points(...)
        let mut u_point = EcPoint::new(&self.group.as_ref())?;
        let mut v_point = EcPoint::new(&self.group.as_ref())?;
        u_point.mul_generator(&self.group.as_ref(), &k, &self.bn_ctx)?;
        v_point.mul(&self.group.as_ref(), &h_point, &k, &self.bn_ctx)?;
        let c = self.hash_points(&[&h_point, &gamma_point, &u_point, &v_point])?;

        let s = &(&k + &(&c * &secret_key)) % &self.order;

        let gamma_string = gamma_point.to_bytes(
            &self.group,
            PointConversionForm::COMPRESSED,
            &mut self.bn_ctx,
        )?;
        let c_string = append_leading_zeros(&c.to_vec(), self.n);
        let s_string = append_leading_zeros(&s.to_vec(), self.qlen);
        // proof =  [Gamma_string||c_string||s_string]
        let proof = [&gamma_string[..], &c_string, &s_string].concat();

        Ok(proof)
    }

    fn verify(&mut self, y: &[u8], pi: &[u8], alpha: &[u8]) -> Result<Vec<u8>, Error> {
        let (gamma_point, c, s) = self.decode_proof(&pi)?;

        let public_key_point = EcPoint::from_bytes(&self.group, &y, &mut self.bn_ctx)?;
        let h_point = self.hash_to_try_and_increment(&public_key_point, alpha)?;

        let mut s_b = EcPoint::new(&self.group.as_ref())?;
        let mut c_y = EcPoint::new(&self.group.as_ref())?;
        let mut u_point = EcPoint::new(&self.group.as_ref())?;
        s_b.mul_generator(&self.group, &s, &self.bn_ctx)?;
        c_y.mul(&self.group, &public_key_point, &c, &self.bn_ctx)?;
        c_y.invert(&self.group, &self.bn_ctx)?;
        u_point.add(&self.group, &s_b, &c_y, &mut self.bn_ctx)?;

        let mut s_h = EcPoint::new(&self.group.as_ref())?;
        let mut c_gamma = EcPoint::new(&self.group.as_ref())?;
        let mut v_point = EcPoint::new(&self.group.as_ref())?;
        s_h.mul(&self.group, &h_point, &s, &self.bn_ctx)?;
        c_gamma.mul(&self.group, &gamma_point, &c, &self.bn_ctx)?;
        c_gamma.invert(&self.group, &self.bn_ctx)?;
        v_point.add(&self.group, &s_h, &c_gamma, &mut self.bn_ctx)?;

        let derived_c = self.hash_points(&[&h_point, &gamma_point, &u_point, &v_point])?;

        if !derived_c.eq(&c) {
            return Err(Error::InvalidProof);
        }
        let beta = self.gamma_to_hash(&gamma_point)?;

        Ok(beta)
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_derive_public_key() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        let secret_key = BigNum::from_slice(&[0x01]).unwrap();
        let public_key = ecdsa.derive_public_key_point(&secret_key).unwrap();
        let public_key_bytes = public_key
            .to_bytes(&ecdsa.group, PointConversionForm::COMPRESSED, &mut ecdsa.bn_ctx)
            .unwrap();

        let expected_point_bytes = vec![
            0x03, 0x6B, 0x17, 0xD1, 0xF2, 0xE1, 0x2C, 0x42, 0x47, 0xF8, 0xBC, 0xE6, 0xE5, 0x63,
            0xA4, 0x40, 0xF2, 0x77, 0x03, 0x7D, 0x81, 0x2D, 0xEB, 0x33, 0xA0, 0xF4, 0xA1, 0x39,
            0x45, 0xD8, 0x98, 0xC2, 0x96,
        ];
        assert_eq!(public_key_bytes, expected_point_bytes);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_prove_p256_sha256_tai_1() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // Secret Key (labelled as x)
        let x = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("73616d706c65").unwrap();

        let pi = ecdsa.prove(&x, &alpha).unwrap();
        let expected_pi = hex::decode("029bdca4cc39e57d97e2f42f88bcf0ecb1120fb67eb408a856050dbfbcbf57c524347fc46ccd87843ec0a9fdc090a407c6fbae8ac1480e240c58854897eabbc3a7bb61b201059f89186e7175af796d65e7").unwrap();
        assert_eq!(pi, expected_pi);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_verify_p256_sha256_tai_1() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // Public Key (labelled as y)
        let y = hex::decode("0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6")
            .unwrap();
        // Ecdsa Proof
        let pi = hex::decode("029bdca4cc39e57d97e2f42f88bcf0ecb1120fb67eb408a856050dbfbcbf57c524347fc46ccd87843ec0a9fdc090a407c6fbae8ac1480e240c58854897eabbc3a7bb61b201059f89186e7175af796d65e7").unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("73616d706c65").unwrap();

        let beta = ecdsa.verify(&y, &pi, &alpha).unwrap();
        let expected_beta =
            hex::decode("59ca3801ad3e981a88e36880a3aee1df38a0472d5be52d6e39663ea0314e594c")
                .unwrap();
        assert_eq!(beta, expected_beta);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "test"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_prove_p256_sha256_tai_2() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // Secret Key (labelled as x)
        let x = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("74657374").unwrap();

        let pi = ecdsa.prove(&x, &alpha).unwrap();
        let expected_pi = hex::decode("03873a1cce2ca197e466cc116bca7b1156fff599be67ea40b17256c4f34ba2549c94ffd2b31588b5fe034fd92c87de5b520b12084da6c4ab63080a7c5467094a1ee84b80b59aca54bba2e2baa0d108191b").unwrap();
        assert_eq!(pi, expected_pi);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "test"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_verify_p256_sha256_tai_2() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // Public Key (labelled as y)
        let y = hex::decode("0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6")
            .unwrap();
        // Ecdsa Proof
        let pi = hex::decode("03873a1cce2ca197e466cc116bca7b1156fff599be67ea40b17256c4f34ba2549c94ffd2b31588b5fe034fd92c87de5b520b12084da6c4ab63080a7c5467094a1ee84b80b59aca54bba2e2baa0d108191b").unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("74657374").unwrap();

        let beta = ecdsa.verify(&y, &pi, &alpha).unwrap();
        let expected_beta =
            hex::decode("dc85c20f95100626eddc90173ab58d5e4f837bb047fb2f72e9a408feae5bc6c1")
                .unwrap();
        assert_eq!(beta, expected_beta);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "Example of ECDSA with ansip256r1 and SHA-256"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_prove_p256_sha256_tai_3() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // Secret Key (labelled as x)
        let x = hex::decode("2ca1411a41b17b24cc8c3b089cfd033f1920202a6c0de8abb97df1498d50d2c8")
            .unwrap();
        // Data to be hashed: ASCII "sample
        let alpha = hex::decode("4578616d706c65206f66204543445341207769746820616e736970323536723120616e64205348412d323536").unwrap();
        let expected_pi = hex::decode("02abe3ce3b3aa2ab3c6855a7e729517ebfab6901c2fd228f6fa066f15ebc9b9d415a680736f7c33f6c796e367f7b2f467026495907affb124be9711cf0e2d05722d3a33e11d0c5bf932b8f0c5ed1981b64").unwrap();
        let pi = ecdsa.prove(&x, &alpha).unwrap();
        assert_eq!(pi, expected_pi);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "Example of ECDSA with ansip256r1 and SHA-256"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_verify_p256_sha256_tai_3() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        // Public Key (labelled as y)
        let y = hex::decode("03596375e6ce57e0f20294fc46bdfcfd19a39f8161b58695b3ec5b3d16427c274d")
            .unwrap();
        // Ecdsa Proof
        let pi = hex::decode("02abe3ce3b3aa2ab3c6855a7e729517ebfab6901c2fd228f6fa066f15ebc9b9d415a680736f7c33f6c796e367f7b2f467026495907affb124be9711cf0e2d05722d3a33e11d0c5bf932b8f0c5ed1981b64").unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("4578616d706c65206f66204543445341207769746820616e736970323536723120616e64205348412d323536").unwrap();

        let beta = ecdsa.verify(&y, &pi, &alpha).unwrap();
        let expected_beta =
            hex::decode("e880bde34ac5263b2ce5c04626870be2cbff1edcdadabd7d4cb7cbc696467168")
                .unwrap();
        assert_eq!(beta, expected_beta);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_hash_to_try_and_increment_1() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        // Public key
        let public_key_hex =
            hex::decode("0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6")
                .unwrap();
        let public_key = EcPoint::from_bytes(&ecdsa.group, &public_key_hex, &mut ecdsa.bn_ctx).unwrap();

        // Data to be hashed with TAI (ASCII "sample")
        let data = hex::decode("73616d706c65").unwrap();
        let hash = ecdsa.hash_to_try_and_increment(&public_key, &data).unwrap();
        let hash_bytes = hash
            .to_bytes(&ecdsa.group, PointConversionForm::COMPRESSED, &mut ecdsa.bn_ctx)
            .unwrap();

        let expected_hash =
            hex::decode("02e2e1ab1b9f5a8a68fa4aad597e7493095648d3473b213bba120fe42d1a595f3e")
                .unwrap();
        assert_eq!(hash_bytes, expected_hash);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "test"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_hash_to_try_and_increment_2() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        // Public key
        let public_key_hex =
            hex::decode("03596375e6ce57e0f20294fc46bdfcfd19a39f8161b58695b3ec5b3d16427c274d")
                .unwrap();
        let public_key = EcPoint::from_bytes(&ecdsa.group, &public_key_hex, &mut ecdsa.bn_ctx).unwrap();

        // Data to be hashed with TAI (ASCII "sample")
        let data = hex::decode("4578616d706c65206f66204543445341207769746820616e736970323536723120616e64205348412d323536").unwrap();
        let hash = ecdsa.hash_to_try_and_increment(&public_key, &data).unwrap();
        let hash_bytes = hash
            .to_bytes(&ecdsa.group, PointConversionForm::COMPRESSED, &mut ecdsa.bn_ctx)
            .unwrap();

        let expected_hash =
            hex::decode("02141e41d4d55802b0e3adaba114c81137d95fd3869b6b385d4487b1130126648d")
                .unwrap();
        assert_eq!(hash_bytes, expected_hash);
    }

    /// Test vector for `K-163` curve
    /// Source: [RFC6979](https://tools.ietf.org/html/rfc6979) (section A.1)
    #[test]
    fn test_generate_nonce_k163() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::K163_SHA256_TAI).unwrap();
        let mut ord = BigNum::new().unwrap();
        ecdsa.group.order(&mut ord, &mut ecdsa.bn_ctx).unwrap();

        // Secret Key (labelled as x)
        let sk = hex::decode("009A4D6792295A7F730FC3F2B49CBC0F62E862272F").unwrap();
        let sk_bn = BigNum::from_slice(&sk).unwrap();

        // Hashed input message (labelled as h1)
        let data = hex::decode("73616d706c65").unwrap();

        // Nonce generation
        let nonce = ecdsa.generate_nonce(&sk_bn, &data).unwrap();

        // Expected result/nonce (labelled as K or T)
        let expected_nonce = hex::decode("023AF4074C90A02B3FE61D286D5C87F425E6BDD81B").unwrap();
        assert_eq!(nonce.to_vec(), expected_nonce);
    }

    /// Test vector for `P-256` curve with `SHA-256`
    /// Message: sample
    /// Source: [RFC6979](https://tools.ietf.org/html/rfc6979) (section A.2.5)
    #[test]
    fn test_generate_nonce_p256_1() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut ord = BigNum::new().unwrap();
        ecdsa.group.order(&mut ord, &mut ecdsa.bn_ctx).unwrap();

        // Secret Key (labelled as x)
        let sk = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        let sk_bn = BigNum::from_slice(&sk).unwrap();

        // Data: ASCII "sample"
        let data = hex::decode("73616d706c65").unwrap();

        // Nonce generation
        let nonce = ecdsa.generate_nonce(&sk_bn, &data).unwrap();

        // Expected result/nonce (labelled as K or T)
        let expected_nonce =
            hex::decode("A6E3C57DD01ABE90086538398355DD4C3B17AA873382B0F24D6129493D8AAD60")
                .unwrap();
        assert_eq!(nonce.to_vec(), expected_nonce);
    }

    /// Test vector for `P-256` curve with `SHA-256`
    /// Message: test
    /// Source: [RFC6979](https://tools.ietf.org/html/rfc6979) (section A.2.5)
    #[test]
    fn test_generate_nonce_p256_2() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut ord = BigNum::new().unwrap();
        ecdsa.group.order(&mut ord, &mut ecdsa.bn_ctx).unwrap();

        // Secret Key (labelled as x)
        let sk = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        let sk_bn = BigNum::from_slice(&sk).unwrap();

        // Data: ASCII "test"
        let data = hex::decode("74657374").unwrap();

        // Nonce generation
        let nonce = ecdsa.generate_nonce(&sk_bn, &data).unwrap();

        // Expected result/nonce (labelled as K or T)
        let expected_nonce =
            hex::decode("D16B6AE827F17175E040871A1C7EC3500192C4C92677336EC2537ACAEE0008E0")
                .unwrap();
        assert_eq!(nonce.to_vec(), expected_nonce);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_generate_nonce_p256_3() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut ord = BigNum::new().unwrap();
        ecdsa.group.order(&mut ord, &mut ecdsa.bn_ctx).unwrap();

        // Secret Key (labelled as x)
        let sk = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        let sk_bn = BigNum::from_slice(&sk).unwrap();

        // Hashed input message (labelled as h1)
        let data =
            hex::decode("02e2e1ab1b9f5a8a68fa4aad597e7493095648d3473b213bba120fe42d1a595f3e")
                .unwrap();

        // Nonce generation
        let nonce = ecdsa.generate_nonce(&sk_bn, &data).unwrap();

        // Expected result/nonce (labelled as K or T)
        let expected_nonce =
            hex::decode("b7de5757b28c349da738409dfba70763ace31a6b15be8216991715fbc833e5fa")
                .unwrap();
        assert_eq!(nonce.to_vec(), expected_nonce);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "test"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_generate_nonce_p256_4() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut ord = BigNum::new().unwrap();
        ecdsa.group.order(&mut ord, &mut ecdsa.bn_ctx).unwrap();

        // Secret Key (labelled as x)
        let sk = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        let sk_bn = BigNum::from_slice(&sk).unwrap();

        // Hashed input message (labelled as h1)
        let data =
            hex::decode("02ca565721155f9fd596f1c529c7af15dad671ab30c76713889e3d45b767ff6433")
                .unwrap();

        // Nonce generation
        let nonce = ecdsa.generate_nonce(&sk_bn, &data).unwrap();

        // Expected result/nonce (labelled as K or T)
        let expected_nonce =
            hex::decode("c3c4f385523b814e1794f22ad1679c952e83bff78583c85eb5c2f6ea6eee2e7d")
                .unwrap();
        assert_eq!(nonce.to_vec(), expected_nonce);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "Example of ECDSA with ansip256r1 and SHA-256"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_generate_nonce_p256_5() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();
        let mut ord = BigNum::new().unwrap();
        ecdsa.group.order(&mut ord, &mut ecdsa.bn_ctx).unwrap();

        // Secret Key (labelled as x)
        let sk = hex::decode("2ca1411a41b17b24cc8c3b089cfd033f1920202a6c0de8abb97df1498d50d2c8")
            .unwrap();
        let sk_bn = BigNum::from_slice(&sk).unwrap();

        // Hashed input message (labelled as h1)
        let data =
            hex::decode("02141e41d4d55802b0e3adaba114c81137d95fd3869b6b385d4487b1130126648d")
                .unwrap();

        // Nonce generation
        let nonce = ecdsa.generate_nonce(&sk_bn, &data).unwrap();

        // Expected result/nonce (labelled as K or T)
        let expected_nonce =
            hex::decode("6ac8f1efa102bdcdcc8db99b755d39bc995491e3f9dea076add1905a92779610")
                .unwrap();
        assert_eq!(nonce.to_vec(), expected_nonce);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_hash_points() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        // Test input data
        let hash_hex =
            hex::decode("02e2e1ab1b9f5a8a68fa4aad597e7493095648d3473b213bba120fe42d1a595f3e")
                .unwrap();
        let pi_hex = hex::decode("029bdca4cc39e57d97e2f42f88bcf0ecb1120fb67eb408a856050dbfbcbf57c524347fc46ccd87843ec0a9fdc090a407c6fbae8ac1480e240c58854897eabbc3a7bb61b201059f89186e7175af796d65e7").unwrap();
        // Compute all required points (gamma, u, v)
        let hash_point = EcPoint::from_bytes(&ecdsa.group, &hash_hex, &mut ecdsa.bn_ctx).unwrap();
        let mut gamma_hex = pi_hex.clone();
        let c_s_hex = gamma_hex.split_off(33);
        let gamma_point = EcPoint::from_bytes(&ecdsa.group, &gamma_hex, &mut ecdsa.bn_ctx).unwrap();
        let u_hex =
            hex::decode("030286d82c95d54feef4d39c000f8659a5ce00a5f71d3a888bd1b8e8bf07449a50")
                .unwrap();
        let u_point = EcPoint::from_bytes(&ecdsa.group, &u_hex, &mut ecdsa.bn_ctx).unwrap();
        let v_hex =
            hex::decode("03e4258b4a5f772ed29830050712fa09ea8840715493f78e5aaaf7b27248efc216")
                .unwrap();
        let v_point = EcPoint::from_bytes(&ecdsa.group, &v_hex, &mut ecdsa.bn_ctx).unwrap();

        let computed_c = ecdsa
            .hash_points(&[&hash_point, &gamma_point, &u_point, &v_point])
            .unwrap();

        let mut expected_c = c_s_hex.clone();
        expected_c.split_off(16);
        assert_eq!(computed_c.to_vec(), expected_c);
    }

    /// Test vector for `P256-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    /// Source: [Ecdsa-draft-05](https://tools.ietf.org/pdf/draft-irtf-cfrg-ecdsa-05) (section A.1)
    #[test]
    fn test_decode_proof() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::P256_SHA256_TAI).unwrap();

        let pi_hex = hex::decode("029bdca4cc39e57d97e2f42f88bcf0ecb1120fb67eb408a856050dbfbcbf57c524347fc46ccd87843ec0a9fdc090a407c6fbae8ac1480e240c58854897eabbc3a7bb61b201059f89186e7175af796d65e7")
            .unwrap();
        let (derived_gamma, derived_c, _) = ecdsa.decode_proof(&pi_hex).unwrap();

        // Expected values
        let mut gamma_hex = pi_hex.clone();
        let c_s_hex = gamma_hex.split_off(33);
        let mut c_hex = c_s_hex.clone();
        c_hex.split_off(16);
        let expected_gamma = EcPoint::from_bytes(&ecdsa.group, &gamma_hex, &mut ecdsa.bn_ctx).unwrap();
        let expected_c = BigNum::from_slice(c_hex.as_slice()).unwrap();

        assert!(derived_c.eq(&expected_c));
        assert!(expected_gamma
            .eq(&ecdsa.group, &derived_gamma, &mut ecdsa.bn_ctx)
            .unwrap());
    }

    /// Test for `SECP256K1-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    #[test]
    fn test_prove_secp256k1_sha256_tai() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        // Secret Key (labelled as x)
        let x = hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
            .unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("73616d706c65").unwrap();

        let pi = ecdsa.prove(&x, &alpha).unwrap();
        let expected_pi = hex::decode("031f4dbca087a1972d04a07a779b7df1caa99e0f5db2aa21f3aecc4f9e10e85d08748c9fbe6b95d17359707bfb8e8ab0c93ba0c515333adcb8b64f372c535e115ccf66ebf5abe6fadb01b5efb37c0a0ec9").unwrap();
        assert_eq!(pi, expected_pi);
    }

    /// Test for `SECP256K1-SHA256-TAI` cipher suite
    /// ASCII: "sample"
    #[test]
    fn test_verify_secp256k1_sha256_tai() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        // Public Key (labelled as y)
        let y = hex::decode("032c8c31fc9f990c6b55e3865a184a4ce50e09481f2eaeb3e60ec1cea13a6ae645")
            .unwrap();
        // Data: ASCII "sample"
        let alpha = hex::decode("73616d706c65").unwrap();
        // Ecdsa proof
        let pi = hex::decode("031f4dbca087a1972d04a07a779b7df1caa99e0f5db2aa21f3aecc4f9e10e85d0814faa89697b482daa377fb6b4a8b0191a65d34a6d90a8a2461e5db9205d4cf0bb4b2c31b5ef6997a585a9f1a72517b6f").unwrap();

        let beta = ecdsa.verify(&y, &pi, &alpha).unwrap();
        let expected_beta =
            hex::decode("612065e309e937ef46c2ef04d5886b9c6efd2991ac484ec64a9b014366fc5d81")
                .unwrap();
        assert_eq!(beta, expected_beta);
    }

    /// Test for false positives in verification:
    /// Verify should fail if the message has changed.
    #[test]
    fn test_verify_secp256k1_sha256_tai_bad_message() {
        let mut ecdsa = ECECDSA::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        // Public Key (labelled as y)
        let y = hex::decode("032c8c31fc9f990c6b55e3865a184a4ce50e09481f2eaeb3e60ec1cea13a6ae645")
            .unwrap();
        // Ecdsa proof
        let pi = hex::decode("031f4dbca087a1972d04a07a779b7df1caa99e0f5db2aa21f3aecc4f9e10e85d0800851b42ee92f76d98c1f19e4a1e855526b20afe0dd6eb232a493adc107eb2b0f1").unwrap();

        // Verify the proof with a different message will fail
        // The original message was "sample"
        let alpha2 = b"notsample".to_vec();
        assert!(ecdsa.verify(&y, &pi, &alpha2).is_err());
    }
}
