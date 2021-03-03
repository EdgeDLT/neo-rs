use crate::usage::{TxAttrUsage, toTxAttrUsage};
use neo_core::stringstream::StringStream;
use std::error::Error;
use crate::txmodel::{ transaction_param};
use neo_core::convert::{num2hexstring, num2VarInt, hex2int};

const maxTransactionAttributeSize: u32 = 65535;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct TransactionAttribute {
    pub(crate) usage: TxAttrUsage,
    pub(crate) data: &'static str,
}

/**
 * An attribute that is used to decorate the transaction.
 * Used for appending additional information to the transaction.
 *
 * For example, a remark is attached as an attribute.
 */
impl transaction_param for TransactionAttribute {
    fn deserialize(&self, hex: &str) -> Result<TransactionAttribute, Error> {
        let ss = StringStream.new(hex);
        self.fromStream(ss)
    }

    fn fromStream(&self, ss: &mut StringStream) -> Result<TransactionAttribute, Error> {
        let usage = hex2int(ss.read(1)?.as_str())?;
        let mut data = "";
        match usage {
            0x00 | 0x30 | 0xa1..=0xaf => data = ss.read(32)?.as_str(),
            0x02 | 0x03 => data = num2hexstring(usage as i32) + ss.read(32)?.as_str(),
            0x20 => data = ss.read(20)?.as_str(),
            0x81 => data = ss.read(hex2int(ss.read(1)?.as_str())? as u32)?.as_str(),
            0x90 | u if u >= 0xf0 => data = ss.readVarBytes()?.as_str(),
            _ => unreachable!()
        }

        Ok(TransactionAttribute { usage:toTxAttrUsage(usage as usize)?, data })
    }

    fn serialize(&self) -> Result<&str, Error> {
        let mut out = num2hexstring(&self.usage as i32);

        match &self.usage as i32 {
            0x81 => out += num2hexstring((&self.data.len() / 2) as i32),
            0x90 | a if a >= 0xf0 => out += num2VarInt((&self.data.len() / 2) as i32),
            0x02 | 0x03 => out += &self.data[2..2 + 64].clone(),
            _ => out += &self.data.clone(),
        }

        Ok(out)
    }

    fn equals(&self, other: &TransactionAttribute) -> bool {
        self.usage == other.usage
            && self.data == other.data
    }

    fn export(&self) -> Result<TransactionAttribute, Error> {
        Ok(TransactionAttribute {
            usage: self.usage.clone(),
            data: self.data.clone(),
        })
    }
}