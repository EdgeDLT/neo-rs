use neo_core::{stringstream::StringStream, key_pair::key_pair};
use neo_crypto::hex;
use std::error::Error;
use neo_core::misc::reverseHex;
use crate::utils::get_asset_id_by_symbol;
use neo_core::fixed8::fixed8;


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct TransactionInput {
  prevHash: &'static str,
  prevIndex: &'static str,
}

/**
 * A reference to a transaction_output in another confirmed transaction.
 * This is used to denote UTXO that will be spent in self transaction.
 */
impl TransactionInput {

  pub fn deserialize(&self, hex: string)-> Result<TransactionInput,Error> {
    let ss = StringStream.new(hex);
    self.fromStream(ss)
  }
  
  pub fn fromStream(&self, mut ss: StringStream) -> Result<TransactionInput,Error> {
    let prevHash = reverseHex(ss.read(32)?.as_str())?;
    let prevIndex = parseInt(reverseHex(ss.read(2)?.as_str())?, 16);
    Ok(TransactionInput{ prevHash, prevIndex })
  }


  pub fn serialize(&self)-> Result<String, Error> {
    reverseHex(self.prevHash) + reverseHex(num2hexstring(self.prevIndex, 2))
  }

  pub fn export(&self)-> TransactionInput {
    TransactionInput {
      prevHash: self.prevHash,
      prevIndex: self.prevIndex,
    }
  }

  pub fn equals(&self, other: &TransactionInput)-> bool{

      self.prevHash == other.prevHash && self.prevIndex == other.prevIndex

  }
}