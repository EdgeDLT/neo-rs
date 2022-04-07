use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;
use crate::Types::PrimitiveType::PrimitiveType;
use crate::Types::Integer::Integer;

pub struct ByteString {}

impl StackItem for ByteString {
    fn Type() -> StackItemType {
        StackItemType::ByteString
    }

    fn convertTo(&self, typ: StackItemType) -> Self {
        todo!()
    }
}

impl PrimitiveType for ByteString {
    fn Memory(&self) -> Vec<u8> {
        todo!()
    }

    fn Size(&self) -> i32 {
        todo!()
    }
}

/// <summary>
/// Represents an immutable memory block in the VM.
/// </summary>
// [DebuggerDisplay("Type={GetType().Name}, Value={System.BitConverter.ToString(Memory.ToArray()).Replace(\"-\", string.Empty)}")]
impl ByteString
{
    /// <summary>
    /// The largest comparable size. If a <see cref="ByteString"/> exceeds this size, comparison operations on it cannot be performed in the VM.
    /// </summary>
    pub const MaxComparableSize: i32 = u16::MAX as i32;

    /// <summary>
    /// An empty <see cref="ByteString"/>.
    /// </summary>
    public static readonly ByteString Empty = ReadOnlyMemory<byte>.Empty;

//     internal override ReadOnlyMemory<byte> Memory { get;
// }
// public override StackItemType Type => StackItemType.ByteString;

    /// <summary>
    /// Create a new <see cref="ByteString"/> with the specified data.
    /// </summary>
    /// <param name="data">The data to be contained in this <see cref="ByteString"/>.</param>
    public ByteString(ReadOnlyMemory<byte> data)
    {
    this.Memory = data;
    }

    public override bool Equals(StackItem? other)
    {
    if (Size > MaxComparableSize)
    throw new InvalidOperationException("The operand exceeds the maximum comparable size.");
    if (ReferenceEquals(this, other)) return true;
    if (other is not ByteString b) return false;
    if (b.Size > MaxComparableSize)
    throw new InvalidOperationException("The operand exceeds the maximum comparable size.");
    return GetSpan().SequenceEqual(b.GetSpan());
    }

    pub fn GetBoolean(&self) -> bool
    {
        if Size > Integer.MaxSize { panic!() }
        // throw new InvalidCastException();
        return Unsafe.NotZero(GetSpan());
    }

    pub fn int GetHashCode()
    {
    unchecked
    {
    int hash = 17;
    foreach (byte element in GetSpan())
    hash = hash * 31 + element;
    return hash;
    }
    }

    public override BigInt GetInteger()
    {
    if (Size > Integer.MaxSize) throw new InvalidCastException( $ "MaxSize exceed: {Size}");
    return new BigInt(GetSpan());
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static implicit operator ReadOnlyMemory<byte>(ByteString value)
    {
    return value.Memory;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static implicit operator ReadOnlySpan<byte>(ByteString value)
    {
    return value.Memory.Span;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static implicit operator ByteString(byte[] value)
    {
    return new ByteString(value);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static implicit operator ByteString(ReadOnlyMemory<byte> value)
    {
    return new ByteString(value);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static implicit operator ByteString(string value)
    {
    return new ByteString(Utility.StrictUTF8.GetBytes(value));
    }
}
}
