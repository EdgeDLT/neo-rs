use crate::Types::PrimitiveType::PrimitiveType;
use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;
use num::BigInt;
use std::intrinsics::size_of;
use core::mem;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Boolean {
    value: bool,
}

impl StackItem for Boolean {
    fn Type() -> StackItemType {
        StackItemType::Boolean
    }

    fn ConvertTo(&self, typ: StackItemType) -> Self {
        todo!()
    }

    fn equals(&self, other: &StackItem) -> bool
    {
        if ReferenceEquals(this, other) { return true; }
        if other.Type() == StackItemType::Boolean
        { return value == b.value; }
        false
    }

    fn GetBoolean(&self) -> bool { self.value }

    fn GetInteger(&self) -> BigInt
    {
        return match self.value == num::Zero {
            false => BigInt.One,
            true => BigInt.Zero
        };
    }
}

impl PrimitiveType for Boolean {
    fn Memory(&self) -> Vec<u8> {
        Vec::from(match self.value == num::Zero {
            false => [1u8],
            true => [0u8]
        })
    }

    fn Size(&self) -> i32 {
        mem::size_of::<bool>() as i32
    }

    fn GetHashCode(&self) -> i32
    {
        return HashCode.Combine(value);
    }
}

/// <summary>
/// Represents a boolean (<see langword="true" /> or <see langword="false" />) value in the VM.
/// </summary>
// [DebuggerDisplay("Type={GetType().Name}, Value={value}")]
impl Boolean
{

    // public override int Size => sizeof(bool);
    // public override StackItemType Type => StackItemType.Boolean;


    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    // public static implicit operator Boolean(bool value)
    // {
    // return new Boolean(value);
    // }
}
