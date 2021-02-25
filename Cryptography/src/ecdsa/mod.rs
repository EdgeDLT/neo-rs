use std::fmt;
use std::{
    fmt::{Debug, Formatter},
    os::raw::c_ulong,
};

use failure::Fail;
use hmac_sha256::HMAC;

use openssl::{
    bn::{BigNum, BigNumContext},
    ec::{EcGroup, EcPoint, PointConversionForm},
    error::ErrorStack,
    hash::{hash, MessageDigest},
    nid::Nid,
};

use crate::hex;

pub trait ecdsa<PublicKey, SecretKey> {
    type Error;

    fn prove(&mut self, x: SecretKey, alpha: &[u8]) -> Result<Vec<u8>, Self::Error>;

    fn verify(&mut self, y: PublicKey, pi: &[u8], alpha: &[u8]) -> Result<Vec<u8>, Self::Error>;
}

use self::utils::{append_leading_zeros, bits2int, bits2octets};

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


impl ecdsa<&[u8], &[u8]> for ECECDSA {
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