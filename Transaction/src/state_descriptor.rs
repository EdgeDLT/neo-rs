use neo_wallet;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use crate::txmodel::{Transaction, Transaction_Trait, TransactionParam};
use neo_core::convert::{num2var_int, hex2int};
use neo_core::stringstream::StringStream;

#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum StateType {
    Account = 0x40,
    Validator = 0x48,
}


fn toStateType(te: usize) -> StateType {
    match StateType::try_from(te) {
        Ok(tp) => tp,
        Err(_) => Err(()),
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct StateDescriptor
{
    state_type: StateType,
    key: &'static str,
    field: &'static str,
    value: &'static str,
}


impl TransactionParam for StateDescriptor {
    fn deserialize(&self, hex: &str) -> Result<StateDescriptor, Error> {
        let ss = StringStream.new(hex);
        self.from_stream(ss)
    }

    fn from_stream(&self, ss: &mut StringStream) -> Result<StateDescriptor, Error> {
        let state_type = hex2int(ss.read(1)?.as_str())?;

        let key = ss.read_var_bytes()?.as_str();
        let field = hexString2str(ss.read_var_bytes());
        let value = ss.read_var_bytes()?.as_str();

        Ok(StateDescriptor { state_type: toStateType(state_type as usize), key, field, value })
    }


    fn serialize(&self) -> Result<&str, Error> {
        let out = num2hexString(&self.state_type);

        out += num2var_int((&self.key.len() / 2) as i32);
        out += self.key.clone();
        let hexField = str2hexString(self.field);
        out += num2var_int(&hexField.len() / 2);
        out += hexField;
        out += num2var_int((&self.value.len() / 2) as i32);
        out += self.value;
        Ok(out)
    }

    fn equals(&self, other: &StateDescriptor) -> bool {
        self.state_type == other.state_type &&
            self.key == other.key &&
            self.field == other.field &&
            self.value == other.value
    }

    fn export(&self) -> Result<StateDescriptor, Error> {
        Ok(StateDescriptor {
            state_type: self.state_type.clone(),
            key: self.key.clone(),
            field: self.field.clone(),
            value: self.value.clone(),
        })
    }
}