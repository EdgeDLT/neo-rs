// #pragma warning disable CS0659
// 
// using System;
// using System.Collections.Generic;
// using System.Numerics;
// using System.Runtime.CompilerServices;

use crate::Types::StackItemType::StackItemType;
use std::any::Any;
use std::error::Error;
use crate::Types::StackItemType::StackItemType::InteropInterface;
use num::BigInt;

// namespace Neo.VM.Types
// {
/// <summary>
/// The base class for all types in the VM.
/// </summary>
pub trait StackItem
{
    fn False() -> bool { false }

    fn IsNull() -> bool { false }

    fn True() -> bool { true }

    fn Type() -> StackItemType;

    fn ConvertTo(&self, typ: StackItemType) -> Self;

    fn DeepCopy(&self) -> &Self
    {
        self.clone()
    }

    fn Equals(&self, other: &StackItem) -> bool
    {
        self == other
    }

    fn FromInterface(&self, value: Option<dyn Any>) -> Result<StackItem, Error>
    {
        if value.is_none() { return Err(()); }

        InteropInterface::new(value)
    }

    fn GetBoolean(&self) -> bool { false }

    fn GetInteger(&self) -> BigInt { panic!() }

    fn GetInterface<T>(&self) -> T
        where T: Self
    {
        panic!()
    }

    fn GetSpan(&self) -> ReadOnlySpan<u8>
    {
        panic!();
    }

    fn GetString(&self) -> String
    {
        return Utility.StrictUTF8.GetString(GetSpan());
    }


// fn static implicit operator StackItem(sbyte value)
// {
// return (Integer)value;
// }
//
//
// fn static implicit operator StackItem(byte value)
// {
// return (Integer)value;
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
