use crate::Types::PrimitiveType::PrimitiveType;
use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;
use num::traits::{Zero, One};
use std::mem::replace;
use num::bigint::{BigUint, BigInt, Sign};
use crate::Types::ReadOnlyMemory::ReadOnlyMemory;

/// <summary>
/// Represents an integer value in the VM.
/// </summary>
// [DebuggerDisplay("Type={GetType().Name}, Value={value}")]

pub struct Integer {
    value: BigInt,
}

impl StackItem for Integer {
    fn Type() -> StackItemType {
        todo!()
    }

    fn ConvertTo(&self, typ: StackItemType) -> Self {
        todo!()
    }
}

impl PrimitiveType for Integer {
    fn Memory(&self) -> Vec<u8> {
        self.value.is_zero()?
        ReadOnlyMemory < byte >.Empty: value.ToByteArray();
    }

    fn Size(&self) -> i32 {
        todo!()
    }
}

impl Integer
{
    /// <summary>
    /// The maximum size of an integer in bytes.
    /// </summary>
    const MaxSize: i32 = 32;
    const Zero: i32 = 0;


    // internal override ReadOnlyMemory<byte> Memory =>
    public override int Size { get;
}
public override StackItemType Type => StackItemType.Integer;

/// <summary>
/// Create an integer with the specified value.
/// </summary>
/// <param name="value">The value of the integer.</param>
public Integer(BigInt value)
{
if (value.IsZero)
{
Size = 0;
}
else
{
Size = value.GetByteCount();
if (Size > MaxSize) throw new ArgumentException( $ "MaxSize exceed: {Size}");
}
this.value = value;
}

public override bool Equals(StackItem? other)
{
if (ReferenceEquals(this, other)) return true;
if (other is Integer i) return value == i.value;
return false;
}

public override bool GetBoolean()
{
return ! value.IsZero;
}

public override int GetHashCode()
{
return HashCode.Combine(value);
}

public override BigInt GetInteger()
{
return value;
}


public static implicit operator Integer(sbyte value)
{
return (BigInt)value;
}


public static implicit operator Integer(byte value)
{
return (BigInt)value;
}


public static implicit operator Integer(short value)
{
return (BigInt)value;
}


public static implicit operator Integer(ushort value)
{
return (BigInt)value;
}


public static implicit operator Integer(int value)
{
return (BigInt)value;
}


public static implicit operator Integer(uint value)
{
return (BigInt)value;
}


public static implicit operator Integer(long value)
{
return (BigInt)value;
}


public static implicit operator Integer(ulong value)
{
return (BigInt)value;
}


public static implicit operator Integer(BigInt value)
{
return new Integer(value);
}
}
