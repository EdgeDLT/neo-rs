use neo_core::{stringstream::StringStream, key_pair::key_pair};
use neo_crypto::hex;
use std::error::Error;
use neo_core::misc::reverseHex;
use crate::utils::get_asset_id_by_symbol;
use neo_core::fixed8::fixed8;


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct TransactionOutput {
    asset_id: &'static str,
    value: fixed8,
    script_hash: &'static str,
}

/**
 * UTXO that is letructed in self transaction.
 * self represents a spendable coin in the system.
 */
impl TransactionOutput {
    pub fn deserialize(&self, hex: &str) -> Result<TransactionOutput, Error> {
        let mut ss = StringStream.new(hex);
        self.fromStream(ss)
    }

    pub fn fromStream(&self, ss: &mut StringStream) -> Result<TransactionOutput, Error> {
        let asset_id = reverseHex(ss.read(32)?.as_str())?;
        let value = Fixed8.fromReverseHex(ss.read(8));
        let script_hash = reverseHex(ss.read(20)?.as_str())?;

        Ok(
            TransactionOutput { asset_id, value, script_hash }
        )
    }

    pub fn fromIntent(
        &self,
        symbol: &str,
        value: fixed8,
        address: &str,
    ) -> Result<TransactionOutput, Error> {
        let asset_id = get_asset_id_by_symbol(symbol)?;
        let script_hash = key_pair::get_addr_hash_from_address(address)?;

        Ok(
            TransactionOutput { asset_id, value, script_hash: hex::encode(script_hash).as_str() }
        )
    }

    pub fn serialize(&self) -> Result<String, Error> {
        reverseHex(self.asset_id) +
            self.value.toReverseHex() +
            reverseHex(self.script_hash)
    }

    pub fn equals(&self, other: &TransactionOutput) -> bool {
        self.asset_id == other.asset_id &&
            self.value.equals(&other.value) &&
            self.script_hash == other.script_hash
    }

    pub fn export(&self) -> TransactionOutput {
        TransactionOutput{
            asset_id: self.asset_id.clone(),
            value: self.value.clone(),
            script_hash: self.script_hash.clone(),
        }
    }
}
