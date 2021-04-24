// #pragma warning disable CS0659
// 
// using System;
// using System.Collections.Generic;
// using System.Numerics;
// using System.Runtime.CompilerServices;

use crate::Types::StackItemType::StackItemType;

// namespace Neo.VM.Types
// {
/// <summary>
/// The base class for all types in the VM.
/// </summary>
pub trait StackItem
{
    /// <summary>
    /// Represents <see langword="false"/> in the VM.
    /// </summary>
    // fn  StackItem False { get; } = new Boolean(false);

    /// <summary>
    /// Indicates whether the object is <see cref="Null"/>.
    /// </summary>
    //  fn IsNull()->bool;

    /// <summary>
    /// Represents <see langword="null"/> in the VM.
    /// </summary>
    // pub static StackItem Null { get; } = new Null();

    /// <summary>
    /// Represents <see langword="true"/> in the VM.
    /// </summary>
    // pub static StackItem True { get; } = new Boolean(true);

    /// <summary>
    /// The type of this VM object.
    /// </summary>
    fn Type() -> StackItemType;

    /// <summary>
    /// Convert the VM object to the specified type.
    /// </summary>
    /// <param name="type">The type to be converted to.</param>
    /// <returns>The converted object.</returns>
    fn ConvertTo(&self, typ: StackItemType) -> Self;
    // {
    //     if (typ == Type) { return *self; }
    //
    //     if (typ == StackItemType.Boolean)
    //     return GetBoolean();
    // }

    /// <summary>
    /// Copy the object and all its children.
    /// </summary>
    /// <returns>The copied object.</returns>
    fn DeepCopy(&self) -> &Self
    {
        self.clone()
    }

    StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap)
    {
    return this;
    }

    fn Equals(&self, obj: &StackItem) -> bool
    {
        if (self == obj) { return true; }
        Equals(item)
    }

    fn Equals(&self, other: &StackItem) -> bool
    {
        self == other
    }

    /// <summary>
    /// Wrap the specified <see cref="object"/> and return an <see cref="InteropInterface"/> containing the <see cref="object"/>.
    /// </summary>
    /// <param name="value">The wrapped <see cref="object"/>.</param>
    /// <returns></returns>
    fn FromInterface(object? value) -> StackItem
    {
        if (value
        is
        null) return Null;
        return new;
        InteropInterface(value);
    }

    /// <summary>
    /// Get the boolean value represented by the VM object.
    /// </summary>
    /// <returns>The boolean value represented by the VM object.</returns>
    fn GetBoolean() -> bool { false }

    /// <summary>
    /// Get the integer value represented by the VM object.
    /// </summary>
    /// <returns>The integer value represented by the VM object.</returns>
    fn GetInteger() -> BigInteger { panic!() }
    // {
    // throw new InvalidCastException();
    // }

    /// <summary>
    /// Get the <see cref="object"/> wrapped by this interface and convert it to the specified type.
    /// </summary>
    /// <typeparam name="T">The type to convert to.</typeparam>
    /// <returns>The wrapped <see cref="object"/>.</returns>
    fn GetInterface<T>() -> T
        where T: Self
    {
        panic!()
        // throw new InvalidCastException();
    }

    /// <summary>
    /// Get the readonly span used to read the VM object data.
    /// </summary>
    /// <returns></returns>
    fn GetSpan() -> ReadOnlySpan<u8>
    {
        panic!();
        // throw new InvalidCastException();
    }

    /// <summary>
    /// Get the <see cref="string"/> value represented by the VM object.
    /// </summary>
    /// <returns>The <see cref="string"/> value represented by the VM object.</returns>
    fn GetString() -> String
    {
        return Utility.StrictUTF8.GetString(GetSpan());
    }


    fn static implicit operator StackItem(sbyte value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(byte value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(short value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(ushort value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(int value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(uint value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(long value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(ulong value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(BigInteger value)
    {
    return (Integer)value;
    }


    fn static implicit operator StackItem(bool value)
    {
    return value ? True: False;
    }


    fn static implicit operator StackItem(byte[] value)
    {
    return (ByteString)value;
    }


    fn static implicit operator StackItem(ReadOnlyMemory<byte> value)
    {
    return (ByteString)value;
    }


    fn static implicit operator StackItem(string value)
    {
    return (ByteString)value;
    }
}
}
