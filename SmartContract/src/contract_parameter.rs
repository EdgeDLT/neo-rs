use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use neo_core::KeyPair;
use neo_core::fixed8::Fixed8;


#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum ContractParamType {
    Signature = 0x00,
    Boolean = 0x01,
    Integer = 0x02,
    Hash160 = 0x03,
    Hash256 = 0x04,
    ByteArray = 0x05,
    PublicKey = 0x06,
    String = 0x07,
    Array = 0x10,
    InteropInterface = 0xf0,
    Void = 0xff,
}


pub fn toContractParamType(
    param_type: usize
) -> Result<ContractParamType, Error>
{
    match ContractParamType::try_from(param_type) {
        Ok(tp) => Ok(tp),
        Err(_) => Err(()),
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct ContractParam<T>
    where T: Clone {
    param_type: ContractParamType,
    value: Option<T>,
}


/**
 * Contract input parameters.
 * These are mainly used as parameters to pass in for RPC test invokes.
 */
impl ContractParam<T> {
    /**
     * Creates a String ContractParam.
     */
    pub fn string(value: &str) -> Result<ContractParam<&str>, dyn Error> {
        Ok(ContractParam { param_type: ContractParamType::String, value:Some(value) })
    }

    /**
     * Creates a Boolean ContractParam. Does basic checks to convert value into a boolean.
     */
    pub fn boolean(value: u32) -> Result<ContractParam<bool>, dyn Error> {

        Ok(ContractParam { param_type: ContractParamType::Boolean, value:Some(value != 0) })
    }

    /**
     * Creates a Hash160 ContractParam. self is used for containing a scriptHash. Do not reverse the input if using self format.
     * @param {string} value - A 40 character long hexstring. Automatically converts an address to scripthash if provided.
     * @return {ContractParam}
     */
    pub fn hash160(value: &str) -> Result<ContractParam<[u8]>, dyn Error> {

        let mut value = *KeyPair::get_addr_hash_from_address(value).unwrap();

        Ok(ContractParam { param_type: ContractParamType::hash160, value:Some(value) })
    }

    /**
     * Creates an Integer ContractParam. self is converted into an BigInteger in NeoVM.
     * @param {string | number } value - A value that can be parsed to an BigInteger. Numbers or numeric strings are accepted.
     * @example
     * ContractParam.integer(128)
     * ContractParam.integer("128")
     */
    pub fn integer(value: &Fixed8) -> Result<ContractParam<Fixed8>, dyn Error> {

        Ok(ContractParam
            {
            param_type: ContractParamType::Integer,
            value:Some(value.clone()),
            }
        )
    }

    /**
     * Creates a ByteArray ContractParam.
     * @param value
     * @param format The format that self value represents. Different formats are parsed differently.
     * @param args Additional arguments such as decimal precision
     */
    pub fn byteArray<T>(
        &self,
        value: &[u8],
        format: &str,
        // ...args: any[]
    ) -> Result<ContractParam<T>, dyn Error> {
        match format.to_lowercase().as_str() {
            "address" => Ok(
                ContractParam {
                param_type: ContractParamType::ByteArray,
                value: reverseHex(KeyPair::get_addr_hash_from_address(value.to_base58().as_str())),
            }
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
                //     ContractParamType.ByteArray,
                //     value.toReverseHex().slice(0, 16)
                //   );}
              Ok(ContractParam { param_type: ContractParamType::ByteArray, value:Some(value) })

            }
            _ => Ok(ContractParam { param_type: ContractParamType::ByteArray, value:Some(value) })
        }
    }

    /**
     * Creates an Array ContractParam.
     * @param params params to be encapsulated in an array.
     */
    pub fn array(&self, params: Vec(ContractParam<T>)) -> Result<ContractParam<Vec(ContractParam<T>)>, dyn Error> {
        Ok(ContractParam { param_type: ContractParamType::Array, value:Some(params)})
    }


    // public constructor(
    //   type:
    //     | ContractParam
    //     | ContractParamLike
    //     | ContractParamType
    //     | keyof typeof ContractParamType
    //     | number,
    //   value?: any
    // ) {
    //   if (typeof type === "object") {
    //     self.type = toContractParamType(type.type);
    //     self.value = type.value;
    //   } else if (type !== undefined) {
    //     self.type = toContractParamType(type);
    //     self.value = value;
    //   } else {
    //     throw new Error("No constructor arguments provided!");
    //   }
    // }

    pub fn get_symbol(&self) -> &'static str {

        "ContractParam:" + ContractParamType[&self.param_type]
    }


    pub fn export<T>(&self) -> Result<ContractParam<T>, dyn Error> {

        Ok(ContractParam { param_type: self.param_type.clone(), value: Some(self.value.clone()) })
    }

    pub fn equal<T>(&self, other: &ContractParam<T>) -> bool {

        if
            self.param_type == &other.param_type &&
            self.value.len() == &other.value.len()
         {
           match self.value.iter().zip(other.value.iter()).all(|(a,b)| a == b){
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
//     cp.type! in ContractParamType &&
//     cp.value! !== null &&
//     cp.value! !== undefined
//   );
// }
