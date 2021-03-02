// import { num2hexstring, num2VarInt, reverseHex, StringStream } from "../../u";
// import { TxAttrUsage } from "../txAttrUsage";

use crate::usage::TxAttrUsage;
use neo_core::stringstream::StringStream;
use std::error::Error;

const maxTransactionAttributeSize:u32 = 65535;

pub struct TransactionAttribute {
    usage: TxAttrUsage,
    data: &'static str,
}

fn toTxAttrUsage(tp: TxAttrUsage) -> TxAttrUsage {
    // if (typeof type == "string") {
    //   if (type in TxAttrUsage) {
    //     return TxAttrUsage[type as keyof typeof TxAttrUsage];
    //   }
    //   throw new Error(`${type} not found in TxAttrUsage!`);
    // }
    tp
}

/**
 * An attribute that is used to decorate the transaction.
 * Used for appending additional information to the transaction.
 *
 * For example, a remark is attached as an attribute.
 */
impl TransactionAttribute {

    pub fn deserialize(&self, hex: string) -> Result<TransactionAttribute, Error> {
        let ss = StringStream.new(hex);
        this.fromStream(ss)
    }

    pub fn fromStream(&self, ss: &mut StringStream) -> Result<TransactionAttribute, Error> {
        let usage = parseInt(ss.read(1), 16);
        let mut data= "";
        match usage {
            0x00 | 0x30 | 0xa1..=0xaf => data = ss.read(32)?.as_str(),
            0x02 | 0x03 => data = num2hexstring(usage) + ss.read(32)?.as_str(),
            0x20 => data = ss.read(20)?.as_str(),
            0x81 => data = ss.read(parseInt(ss.read(1), 16))?.as_str(),
            0x90 | u if u >= 0xf0 => data = ss.readVarBytes()?.as_str(),
            _ => unreachable!()
        }

        Ok(TransactionAttribute { usage, data })
    }

    pub fn serialize(&self) -> Result<String, Error>{

    let out = num2hexstring(self.usage as u32);

    if (this.usage == 0x81) {
    out += num2hexstring(this.data.length / 2);
    } else if (this.usage == 0x90 | | this.usage > = 0xf0) {
    out += num2VarInt(this.data.length / 2);
    }
    if (this.usage == 0x02 | | this.usage == 0x03) {
    out += this.data.substr(2, 64);
    } else {
    out += this.data;
    }
    return out;
    }

    pub fn export(&self) -> Result<TransactionAttribute, Error> {
    Ok(TransactionAttribute {
    usage: this.usage,
    data: this.data,
    })
    }

    pub fn equals(&self, other:&TransactionAttribute)-> bool {
    return (
    this.usage == toTxAttrUsage(other.usage) & & this.data == other.data
    );
    }
}

export default TransactionAttribute;
