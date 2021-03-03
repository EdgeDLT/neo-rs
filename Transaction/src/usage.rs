
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

/**
 * Enum for TransactionAttributeUsage
 * @enum {number}
 */
#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum TxAttrUsage {
  ContractHash = 0,
  ECDH02 = 2,
  ECDH03 = 3,
  Script = 32,
  Vote = 48,
  DescriptionUrl = 129,
  Description = 144,
  Hash1 = 161,
  Hash2 = 162,
  Hash3 = 163,
  Hash4 = 164,
  Hash5 = 165,
  Hash6 = 166,
  Hash7 = 167,
  Hash8 = 168,
  Hash9 = 169,
  Hash10 = 170,
  Hash11 = 171,
  Hash12 = 172,
  Hash13 = 173,
  Hash14 = 174,
  Hash15 = 175,
  Remark = 240,
  Remark1 = 241,
  Remark2 = 242,
  Remark3 = 243,
  Remark4 = 244,
  Remark5 = 245,
  Remark6 = 246,
  Remark7 = 247,
  Remark8 = 248,
  Remark9 = 249,
  Remark10 = 250,
  Remark11 = 251,
  Remark12 = 252,
  Remark13 = 253,
  Remark14 = 254,
  Remark15 = 255,
}

impl fmt::Display for TxAttrUsage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub fn toTxAttrUsage(tp: usize) -> Result<TxAttrUsage, Error> {
   match TxAttrUsage::try_from(te) {
        Ok(tp) => Ok(tp),
        Err(_) => Err(()),
    }
}
