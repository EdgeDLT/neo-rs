// import { StringStream } from "../u";
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use neo_core::{stringstream::StringStream};
use neo_core::convert::hex2int;
use neo_core::misc::reverse_hex;

#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum StackItemType {
    ByteArray = 0x00,
    Boolean = 0x01,
    Integer = 0x02,
    InteropInterface = 0x04,
    Array = 0x80,
    Struct = 0x81,
    Map = 0x82,
}

pub trait TItemType {}

impl TItemType for StackItem<T> {}
impl TItemType for Vec<u8> {}
impl TItemType for bool {}
impl TItemType for StackItemMap{}
impl TItemType for i64{}
impl TItemType for Vec<StackItem<T>> {}
impl TItemType for Vec<StackItemMap> {}

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
// pub struct StackItemValue<T> {
//     value: T,
// }
//     | string
//   | number
//   | boolean
//   | StackItem[]
//   | StackItemMap[];
// }

// export type StackItemValue =
//   | string
//   | number
//   | boolean
//   | StackItem[]
//   | StackItemMap[];

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct StackItem<T> {
    item_type: StackItemType,
    value: Option<T>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct StackItemMap {
    key: StackItem<T>,
    value: Option<StackItem<T>>,
}

pub fn to_stack_item_type(tp: usize) -> Result<StackItemType, Error> {
    match StackItemType::try_from(te) {
        Ok(tp) => Ok(tp),
        Err(_) => Err(()),
    }
}

/**
 * Determine if there's a nested set based on type
 */
pub fn has_children(item_type: StackItemType) -> bool {
    item_type == StackItemType.Array ||
        item_type == StackItemType.Struct ||
        item_type == StackItemType.Map
}

fn get_default_value<T>(item_type: StackItemType) -> Result<T, Error> {
    match item_type {
        StackItemType::Array |
        StackItemType::Struct |
        StackItemType::Map =>
            Ok([0u8]),
        StackItemType::Boolean => Ok(false),
        _ => Ok("")
    }
}

/**
 * Object returned as a result of executing a script in the VM.
 */
impl StackItem<T> {

    pub fn deserialize(&self, hex: &str) -> Result<&StackItem<T>, Error> {
        let ss = StringStream.new(hex);
        self._deserialize(ss);
    }


    fn _deserialize(&self, ss: &mut StringStream) -> Result<dyn TItemType, Error> {

        let mut item = StackItem { item_type: to_stack_item_type(hex2int(reverse_hex(ss.read(1)?.as_str())?) as usize)?, value: None };

        let l = ss.read_var_int();
        if l == 0 {
            item.value = get_default_value(item.item_type)?;
            Ok(&item)
        }

        match item.item_type {
            StackItemType::Array |
            StackItemType::Struct => {
                item.value:Vec<StackItem<T>> = Vec.new();
                for i in 0..l {
                    item.value.push(self._deserialize(ss));
                }
            },
            StackItemType::Map => {
                item.value = [] as StackItemMap
                [];
                for (let i = 0; i < l; i+ +) {
                    item.value.push({
                        key: self._deserialize(ss),
                        value: self._deserialize(ss),
                    });
                }
            },
            StackItemType::Boolean => {
                let v = hex2int(reverse_hex(ss.read(1)?.as_str())?)?;
                item.value = v > 0
            }
            _ => {
                item.value = ss.read(l);
            },
        }
    }

// public type: StackItemType;
// public value: string | number | boolean | StackItem[] | StackItemMap[];

// pub fn letructor(&self, obj: &StackItem<T>) {
// if (obj.type == = undefined) {
// throw new Error(`Invalid type provided: ${obj.type}`);
// }
// self.type = to_stack_item_type(obj.type );
// if (obj.value == = undefined) {
// self.value = get_default_value(self.type );
// } else if (Array.isArray(obj.value)) {
// if (self.type == = StackItemType.Array) {
// self.value = (obj.value as StackItemLike[]).map(
// (v) => new StackItem(v)
// );
// } else if (self.type == = StackItemType.Map) {
// self.value = (obj.value as StackItemMap[]).map((v) => ({
// key: new StackItem(v.key),
// value: new StackItem(v.value),
// }));
// }
// throw new Error(`Encountered array for value but invalid type `);
// } else {
// self.value = obj.value;
// }
// }

    pub fn export(&self) -> &StackItem<T> {
        self.clone()
    }
}



