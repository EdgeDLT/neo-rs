use neo_core::{stringstream::StringStream, KeyPair::KeyPair};
use neo_crypto::hex;
use std::error::Error;
use neo_core::misc::reverseHex;
use crate::utils::get_asset_id_by_symbol;
use neo_core::fixed8::fixed8;
use crate::txmodel::{Transaction, Transaction_Trait, transaction_param};
use neo_core::convert::{num2hexstring, hex2int};


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct TransactionInput {
    prevHash: &'static str,
    prevIndex: u32,
}

/**
 * A reference to a transaction_output in another confirmed transaction.
 * This is used to denote UTXO that will be spent in self transaction.
 */
impl transaction_param for TransactionInput {
    fn deserialize(&self, hex: string) -> Result<TransactionInput, Error> {
        let ss = StringStream.new(hex);
        self.fromStream(ss)
    }

    fn fromStream(&self, ss: &mut StringStream) -> Result<TransactionInput, Error> {
        let prevHash = reverseHex(ss.read(32)?.as_str())?;
        let prevIndex = hex2int(reverseHex(ss.read(2)?.as_str())?)?;
        Ok(TransactionInput { prevHash, prevIndex: prevIndex as u32 })
    }

    fn serialize(&self) -> Result<String, Error> {
        reverseHex(self.prevHash) + reverseHex(num2hexstring(self.prevIndex as i32))
    }

    fn equals(&self, other: &TransactionInput) -> bool {
        self.prevHash == other.prevHash && self.prevIndex == other.prevIndex
    }

    fn export(&self) -> TransactionInput {
        TransactionInput {
            prevHash: self.prevHash,
            prevIndex: self.prevIndex,
        }
    }
}