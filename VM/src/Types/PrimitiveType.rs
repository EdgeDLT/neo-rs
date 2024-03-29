use crate::Types::StackItem::StackItem;

/// <summary>
    /// The base class for primitive types in the VM.
    /// </summary>
     pub trait PrimitiveType : StackItem
    {
        fn  Memory(&self)->Vec<u8>;

        /// <summary>
        /// The size of the VM object in bytes.
        /// </summary>
        fn  Size(&self)->i32;// => Memory.Length;

        // fn override StackItem ConvertTo(StackItemType type)
        // {
        //     if (type == Type) return this;
        //     return type switch
        //     {
        //         StackItemType.Integer => GetInteger(),
        //         StackItemType.ByteString => Memory,
        //         StackItemType.Buffer => new Buffer(GetSpan()),
        //         _ => base.ConvertTo(type)
        //     };
        // }

        // fn StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap)
        // {
        //     return this;
        // }

        // fn abstract override bool Equals(StackItem? other);

        /// <summary>
        /// Get the hash code of the VM object, which is used for key comparison in the <see cref="Map"/>.
        /// </summary>
        /// <returns>The hash code of this VM object.</returns>
        fn  hash_code(&self)->i32{panic!()}

        // fn sealed override ReadOnlySpan<byte> GetSpan()
        // {
        //     return Memory.Span;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(sbyte value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(byte value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(short value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(ushort value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(int value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(uint value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(long value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(ulong value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(BigInt value)
        // {
        //     return (Integer)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(bool value)
        // {
        //     return (Boolean)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(byte[] value)
        // {
        //     return (ByteString)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(ReadOnlyMemory<byte> value)
        // {
        //     return (ByteString)value;
        // }
        //
        //
        // fn  implicit operator PrimitiveType(string value)
        // {
        //     return (ByteString)value;
        // }
    }
