use std::error::Error;

pub trait KeyTrait {
    fn deserialize(&self, hex: &str) -> Result<dyn KeyTrait, dyn Error>;

    fn serialize(&self) -> Result<String, dyn Error>;

    fn to_hex(&self) -> String;

    fn to_slice(&self) -> &[u8];

    fn equals(&self, other: &dyn KeyTrait) -> bool;
}