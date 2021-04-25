use std::convert::TryFrom;
use std::error::Error;

use neo_core::convert::ab2hexstring;
use neo_core::fixed8::Fixed8;
use neo_core::KeyPair;
use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum ContractParam {
    Signature(String) = 0x00,
    Boolean(bool) = 0x01,
    Integer(Fixed8) = 0x02,
    Hash160(String) = 0x03,
    Hash256(String) = 0x04,
    ByteArray([u8]) = 0x05,
    PublicKey(String) = 0x06,
    String(String) = 0x07,
    Array(Vec<&'static ContractParam>) = 0x10,
    InteropInterface = 0xf0,
    Void = 0xff,
}


pub fn to_contract_param_type(
    param_type: usize
) -> Result<ContractParam, Error>
{
    match ContractParam::try_from(param_type) {
        Ok(tp) => Ok(tp),
        Err(_) => Err(()),
    }
}

// match param {
//      ContractParamT::Signature(val) => val,
//      ContractParamT::Boolean(val) => val,
//      ContractParamT::Integer(val) => val,
//      ContractParamT::Hash160(val) => val,
//      ContractParamT::Hash256(val) => val,
//      ContractParamT::ByteArray(val) => val,
//      ContractParamT::PublicKey(val) => val,
//      ContractParamT::String(val) => val,
//      ContractParamT::Array(val) => val,
//      ContractParamT::InteropInterface => val,
//      ContractParamT::Void => val,
//      _ => unreachable!(),
//  }

// #[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
// pub struct ContractParam {
//
//     value: Option<T>,
// }


/// Get methods
impl ContractParam {
    pub fn param_type(&self) -> &ContractParam {
        &self.param_type
    }
    pub fn value(&self) -> &Option<T> {
        &self.value
    }
}


/**
 * Contract input parameters.
 * These are mainly used as parameters to pass in for RPC test invokes.
 */
impl ContractParam {
    /**
     * Creates a String ContractParam.
     */
    pub fn string(value: &str) -> ContractParam {
        ContractParam::String(value.to_string())
    }

    /**
     * Creates a Boolean ContractParam. Does basic checks to convert value into a boolean.
     */
    pub fn boolean(value: u32) -> ContractParam {
        ContractParam::Boolean(value != 0)
    }

    /**
     * Creates a Hash160 ContractParam. self is used for containing a scriptHash. Do not reverse the input if using self format.
     * @param {string} value - A 40 character long hexstring. Automatically converts an address to scripthash if provided.
     * @return {ContractParam}
     */
    pub fn hash160(value: &str) -> ContractParam {
        let mut value = *KeyPair::get_addr_hash_from_address(value).unwrap();

        ContractParam::hash160(ab2hexstring(&value).as_str())
    }

    /**
     * Creates an Integer ContractParam. self is converted into an BigInt in NeoVM.
     * @param {string | number } value - A value that can be parsed to an BigInt. Numbers or numeric strings are accepted.
     * @example
     * ContractParam.integer(128)
     * ContractParam.integer("128")
     */
    pub fn integer(value: &Fixed8) -> ContractParam {
        ContractParam::Integer(Fixed8(value.0))
    }

    /**
     * Creates a ByteArray ContractParam.
     * @param value
     * @param format The format that self value represents. Different formats are parsed differently.
     * @param args Additional arguments such as decimal precision
     */
    pub fn byteArray(
        &self,
        value: &[u8],
        format: &str,
        // ...args: any[]
    ) -> ContractParam {
        match format.to_lowercase().as_str() {
            "address" =>
                ContractParam::ByteArray(
                    reverseHex(
                        KeyPair::get_addr_hash_from_address(
                            value.to_base58().as_str()
                        )
                    )
                ),

            "Fixed8" => {
                //TODO:
                // let decimals = 8;
                //   if (args.length === 1) {
                //     decimals = args[0];
                //   }
                //   if (!isFinite(value)) {
                //     throw new Error(`Input should be number!`);
                //   }
                //   const divisor = new Fixed8(Math.pow(10, 8 - decimals));
                //   const fixed8Value = new Fixed8(value);
                //   const adjustedValue = fixed8Value.times(Math.pow(10, decimals));
                //   const modValue = adjustedValue.mod(1);
                //   if (!modValue.isZero()) {
                //     throw new Error(`wrong precision: expected ${decimals}`);
                //   }
                //   value = fixed8Value.div(divisor);
                //   return new ContractParam(
                //     ContractParam.ByteArray,
                //     value.toReverseHex().slice(0, 16)
                //   );}
                ContractParam::ByteArray(*value)
            }
            _ => ContractParam::ByteArray(*value),
        }
    }

    /**
     * Creates an Array ContractParam.
     * @param params params to be encapsulated in an array.
     */
    pub fn array(&self, params: &[ContractParam]) -> ContractParam {
        let mut arr = Vec::new();
        for param in params {
            arr.push(param.clone());
        }
        ContractParam::Array(arr)
    }


    // public constructor(
    //   type:
    //     | ContractParam
    //     | ContractParamLike
    //     | ContractParam
    //     | keyof typeof ContractParam
    //     | number,
    //   value?: any
    // ) {
    //   if (typeof type === "object") {
    //     self.type = to_contract_param_type(type.type);
    //     self.value = type.value;
    //   } else if (type !== undefined) {
    //     self.type = to_contract_param_type(type);
    //     self.value = value;
    //   } else {
    //     throw new Error("No constructor arguments provided!");
    //   }
    // }

    pub fn get_symbol(&self) -> &'static str {
        "ContractParam:" + ContractParam[&self.param_type]
    }


    pub fn export(&self) -> &self {
        // ContractParam { param_type: self.param_type.clone(), value: Some(self.value.clone()) })
        &self
    }

    pub fn equal<T>(&self, other: &ContractParam) -> bool {
        if
        self as u32 == &other as u32 &&
            self.value.len() == &other.value.len()
        {
            match self.value.iter().zip(other.value.iter()).all(|(a, b)| a == b) {
                Ok(res) => true,
                Err(e) => false,
            }
        }
        false
    }
}

// pub fn likeContractParam(&self, cp: Partial<ContractParam>)-> bool {
//   if (cp === null || cp === undefined) {
//     return false;
//   }
//   if (cp instanceof ContractParam) {
//     return true;
//   }
//   return (
//     cp.type! in ContractParam &&
//     cp.value! !== null &&
//     cp.value! !== undefined
//   );
// }
