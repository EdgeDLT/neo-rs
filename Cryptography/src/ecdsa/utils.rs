use openssl::{
    bn::{BigNum, BigNumContext},
    error::ErrorStack,
};
use crate::hex;

pub fn append_leading_zeros(data: &[u8], bits_length: usize) -> Vec<u8> {
    if data.len() * 8 > bits_length {
        return data.to_vec();
    }

    let leading_zeros = if bits_length % 8 > 0 {
        vec![0; bits_length / 8 - data.len() + 1]
    } else {
        vec![0; bits_length / 8 - data.len()]
    };

    [&leading_zeros[..], &data].concat()
}

pub fn bits2int(data: &[u8], qlen: usize) -> Result<BigNum, ErrorStack> {
    let data_len_bits = data.len() * 8;
    let result = BigNum::from_slice(data).and_then(|data_bn| {
        if data_len_bits > qlen {
            let mut truncated = BigNum::new()?;
            truncated.rshift(&data_bn, (data_len_bits - qlen) as i32)?;

            Ok(truncated)
        } else {
            Ok(data_bn)
        }
    })?;
    let _data2 = data.to_vec();
    let _data_vec = result.to_vec();

    Ok(result)
}

pub fn bits2octets(
    data: &[u8],
    length: usize,
    order: &BigNum,
    bn_ctx: &mut BigNumContext,
) -> Result<Vec<u8>, ErrorStack> {
    let z1 = bits2int(data, length)?;
    let result = BigNum::new().and_then(|mut res| {
        res.nnmod(&z1, order, bn_ctx)?;
        Ok(res.to_vec())
    })?;

    Ok(result)
}