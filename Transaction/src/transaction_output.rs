use neo_core::{stringstream::StringStream, KeyPair::KeyPair};
use neo_crypto::hex;
use std::error::Error;
use neo_core::misc::reverse_hex;
use crate::utils::get_asset_id_by_symbol;
use neo_core::fixed8::Fixed8;
use crate::txmodel::{Transaction, Transaction_Trait, transaction_param};


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct TransactionOutput {
    asset_id: &'static str,
    value: Fixed8,
    script_hash: &'static str,
}


impl TransactionOutput {

    pub fn fromIntent(
        &self,
        symbol: &str,
        value: Fixed8,
        address: &str,
    ) -> Result<TransactionOutput, Error> {
        let asset_id = get_asset_id_by_symbol(symbol)?;
        let script_hash = KeyPair::get_addr_hash_from_address(address)?;

        Ok(
            TransactionOutput { asset_id, value, script_hash: hex::encode(script_hash).as_str() }
        )
    }
}

/**
 * UTXO that is letructed in self transaction.
 * self represents a spendable coin in the system.
 */
impl transaction_param for TransactionOutput {

    fn deserialize(&self, hex: &str) -> Result<TransactionOutput, Error> {
        let mut ss = StringStream.new(hex);
        self.fromStream(ss)
    }

    fn fromStream(&self, ss: &mut StringStream) -> Result<TransactionOutput, Error> {
        let asset_id = reverse_hex(ss.read(32)?.as_str())?;
        let value = Fixed8.fromReverseHex(ss.read(8));
        let script_hash = reverse_hex(ss.read(20)?.as_str())?;

        Ok(
            TransactionOutput { asset_id, value, script_hash }
        )
    }


    fn serialize(&self) -> Result<String, Error> {
        reverse_hex(self.asset_id) +
            self.value.toReverseHex() +
            reverse_hex(self.script_hash)
    }

    fn equals(&self, other: &TransactionOutput) -> bool {
        self.asset_id == other.asset_id &&
            self.value.equals(&other.value) &&
            self.script_hash == other.script_hash
    }

    fn export(&self) -> TransactionOutput {
        TransactionOutput {
            asset_id: self.asset_id.clone(),
            value: self.value.clone(),
            script_hash: self.script_hash.clone(),
        }
    }
}


