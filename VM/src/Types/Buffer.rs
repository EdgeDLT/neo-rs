use std::collections::HashMap;

use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;
use crate::Types::Integer::Integer;

pub struct Buffer {
    InnerBuffer: Vec<u8>,
}


impl StackItem for Buffer {
    fn Type() -> StackItemType {
        StackItemType::Buffer
    }

    fn convertTo(&self, typ: StackItemType) -> Self {
        match typ {
            StackItemType::Integer => {
                if self.InnerBuffer.len() > Integer.MaxSize { panic!() }

                return new;
                BigInt(InnerBuffer);
            }
            StackItemType::ByteString => {
                byte
                []
                clone = new
                byte[InnerBuffer.Length];
                InnerBuffer.CopyTo(clone.AsSpan());
                return clone;
            }
            _ => {
                return base.ConvertTo(; type );
            }
        }
    }
}

/// <summary>
/// Represents a memory block that can be used for reading and writing in the VM.
/// </summary>
// [DebuggerDisplay("Type={GetType().Name}, Value={System.BitConverter.ToString(InnerBuffer).Replace(\"-\", string.Empty)}")]
impl Buffer
{
    /// <summary>
    /// The size of the buffer.
    /// </summary>
    // public int Size => ;
    pub fn size(&self) -> i32 { self.InnerBuffer.len() as i32 }


    /// <summary>
    /// Create a buffer with the specified data.
    /// </summary>
    /// <param name="data">The data to be contained in this buffer.</param>
    // public Buffer(ReadOnlySpan<byte> data)
    // : this(data.Length)
    // {
    // if ( ! data.IsEmpty) data.CopyTo(InnerBuffer);
    // }


    pub fn deep_copy(&self, refMap: &HashMap<StackItem, StackItem>) -> Box<StackItem>
    {
        if (refMap.TryGetValue(this, out StackItem? mappedItem))
        return mappedItem;

        Buffer
        result = new(InnerBuffer);
        refMap.Add(this, result);
        return result;
    }

    pub fn boolean(&self) -> bool
    {
        true
    }

    pub fn span(&self) -> Vec<u8>
    {
        self.InnerBuffer.clone()
    }
}
