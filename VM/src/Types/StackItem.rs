use crate::Types::StackItemType::StackItemType;
use std::any::Any;
use std::error::Error;
use crate::Types::StackItemType::StackItemType::InteropInterface;
use num::BigInt;

/// <summary>
/// The base class for all types in the VM.
/// </summary>
pub trait StackItem
{
    fn False() -> bool { false }

    fn is_null() -> bool { false }

    fn True() -> bool { true }

    fn Type() -> StackItemType;

    fn convertTo(&self, typ: StackItemType) -> Self;

    fn deep_copy(&self) -> &Self
    {
        self.clone()
    }

    fn equals(&self, other: &StackItem) -> bool
    {
        self == other
    }

    fn from_interface(&self, value: Option<dyn Any>) -> Result<StackItem, Error>
    {
        if value.is_none() { return Err(()); }
        InteropInterface::new(value)
    }

    fn boolean(&self) -> bool { false }

    fn integer(&self) -> BigInt { panic!() }

    fn interface<T>(&self) -> T
        where T: Self
    {
        panic!()
    }

    fn span(&self) -> ReadOnlySpan<u8>
    {
        panic!();
    }

    fn string(&self) -> String
    {
        return Utility.StrictUTF8.GetString(GetSpan());
    }


    // fn  from_sbyte(value:sbyte)-> Box<Self>
    // {
    //     Self{value}
    // }


// fn from_byte(value:byte)-> Box<Self>
// {
//     Self{value}
// }
//
//
// fn static implicit operator StackItem(short value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(ushort value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(int value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(uint value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(long value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(ulong value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(BigInt value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(bool value)
// {
// return value ? True: False;
// }
//
//
// fn static implicit operator StackItem(byte[] value)
// {
// return (ByteString)value;
// }
//
//
// fn static implicit operator StackItem(ReadOnlyMemory<byte> value)
// {
// return (ByteString)value;
// }
//
//
// fn static implicit operator StackItem(string value)
// {
// return (ByteString)value;
// }
}
