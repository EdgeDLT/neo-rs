use neo_core::{stringstream::StringStream, KeyPair::KeyPair};
use neo_crypto::hex;
use std::error::Error;
use neo_core::misc::reverse_hex;
use crate::utils::get_asset_id_by_symbol;
use neo_core::fixed8::Fixed8;
use crate::txmodel::{Transaction, Transaction_Trait, TransactionParam};
use neo_core::convert::{num2hexstring, hex2int};


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct TransactionInput {
    prevHash: &'static str,
    prevIndex: u32,
}

/**
 * A reference to a transaction_output in another confirmed Transaction.
 * This is used to denote UTXO that will be spent in self Transaction.
 */
impl TransactionParam for TransactionInput {
    fn deserialize(&self, hex: string) -> Result<TransactionInput, Error> {
        let ss = StringStream.new(hex);
        self.from_stream(ss)
    }

    fn from_stream(&self, ss: &mut StringStream) -> Result<TransactionInput, Error> {
        let prevHash = reverse_hex(ss.read(32)?.as_str())?;
        let prevIndex = hex2int(reverse_hex(ss.read(2)?.as_str())?)?;
        Ok(TransactionInput { prevHash, prevIndex: prevIndex as u32 })
    }

    fn serialize(&self) -> Result<String, Error> {
        reverse_hex(self.prevHash) + reverse_hex(num2hexstring(self.prevIndex as i32))
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