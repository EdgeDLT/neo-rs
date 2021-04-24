/// <summary>
/// Represents an integer value in the VM.
/// </summary>
// [DebuggerDisplay("Type={GetType().Name}, Value={value}")]

pub struct Integer {
    value: BigInteger,
}

impl PrimitiveType for Integer {}

impl Integer
{
    /// <summary>
    /// The maximum size of an integer in bytes.
    /// </summary>
    const MaxSize: i32 = 32;
    const Zero: i32 = 0;
    private readonly  value;

    internal override ReadOnlyMemory<byte> Memory => value.IsZero ? ReadOnlyMemory<byte>.Empty : value.ToByteArray();
    public override int Size { get;
}
public override StackItemType Type => StackItemType.Integer;

/// <summary>
/// Create an integer with the specified value.
/// </summary>
/// <param name="value">The value of the integer.</param>
public Integer(BigInteger value)
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

public override BigInteger GetInteger()
{
return value;
}


public static implicit operator Integer(sbyte value)
{
return (BigInteger)value;
}


public static implicit operator Integer(byte value)
{
return (BigInteger)value;
}


public static implicit operator Integer(short value)
{
return (BigInteger)value;
}


public static implicit operator Integer(ushort value)
{
return (BigInteger)value;
}


public static implicit operator Integer(int value)
{
return (BigInteger)value;
}


public static implicit operator Integer(uint value)
{
return (BigInteger)value;
}


public static implicit operator Integer(long value)
{
return (BigInteger)value;
}


public static implicit operator Integer(ulong value)
{
return (BigInteger)value;
}


public static implicit operator Integer(BigInteger value)
{
return new Integer(value);
}
}
