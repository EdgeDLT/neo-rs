use num::BigInt;
use num::bigint::Sign;
use crate::Memory::Memory;

use crate::OpCode::OpCode;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ScriptBuilder {
    ms: Memory,
    writer: BinaryWriter,
}

// <summary>
/// A helper class for building scripts.
/// </summary>
impl ScriptBuilder
{
    /// <summary>
    /// Initializes a new instance of the <see cref="ScriptBuilder"/> class.
    /// </summary>
    pub fn New() -> Self {
        let mut writer = BinaryWriter(ms);
        let mut ms = Memory();
        Self {
            ms,
            writer,
        }
    }
    /// <summary>
    /// The length of the script.
    /// </summary>
    pub fn Length(&self) -> i32 {
        self.ms.Position
    }

    // public void Dispose()
    // {
    // writer.Dispose();
    // ms.Dispose();
    // }

    /// <summary>
    /// Emits an <see cref="Instruction"/> with the specified <see cref="OpCode"/> and operand.
    /// </summary>
    /// <param name="opcode">The <see cref="OpCode"/> to be emitted.</param>
    /// <param name="operand">The operand to be emitted.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn Emit(&mut self, opcode: OpCode, operand: Option<byte>) -> &mut ScriptBuilder
    {
        self.writer.Write(opcode);
        match operand {
            // The division was valid
            Some(x) => writer.Write(x),
            // The division was invalid
            None => {},
        }
        self
    }

    /// <summary>
    /// Emits a call <see cref="Instruction"/> with the specified offset.
    /// </summary>
    /// <param name="offset">The offset to be called.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitCall(&mut self, offset: i32) -> &mut ScriptBuilder
    {
        // if (offset < sbyte.MinValue | | offset > sbyte.MaxValue)

        self.Emit(OpCode::CALL_L, BitConverter.GetBytes(offset));
        self
        // else
        // return Emit(OpCode.CALL, new[] { (byte)offset });
    }

    /// <summary>
    /// Emits a jump <see cref="Instruction"/> with the specified offset.
    /// </summary>
    /// <param name="opcode">The <see cref="OpCode"/> to be emitted. It must be a jump <see cref="OpCode"/></param>
    /// <param name="offset">The offset to jump.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitJump(&mut self, mut opcode: OpCode, offset: i32) -> &mut ScriptBuilder
    {
        if opcode < OpCode::JMP || opcode > OpCode::JMPLE_L {
            panic!();
        }

        if opcode % 2 == 0 && (offset < sbyte.MinValue || offset > sbyte.MaxValue) {
            opcode.0 = opcode.0 + 1; /// TODO: Check this
        }

        if opcode % 2 == 0 {
            self.Emit(opcode, StackItem { offset })
        } else {
            self.Emit(opcode, BitConverter.GetBytes(offset))
        }
        self
    }

    /// <summary>
    /// Emits a push <see cref="Instruction"/> with the specified number.
    /// </summary>
    /// <param name="value">The number to be pushed.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitPush(&mut self, value: BigInt) -> &mut ScriptBuilder
    {
        if value >= BigInt::from(-1) && value <= BigInt::from(16) {
            self.Emit(OpCode::PUSH0 + (byte)(int)value)
        };

        let data = value.ToByteArray(false, false);
        if data.Length == 1 {
            Emit(OpCode::PUSHINT8, data);
        }

        if data.Length == 2 { self.Emit(OpCode::PUSHINT16, data); }
        if data.Length <= 4 { self.Emit(OpCode::PUSHINT32, PadRight(data, 4)); }
        if data.Length <= 8 { self.Emit(OpCode::PUSHINT64, PadRight(data, 8)); }
        if data.Length <= 16 { self.Emit(OpCode::PUSHINT128, PadRight(data, 16)); }
        if data.Length <= 32 { self.Emit(OpCode::PUSHINT256, PadRight(data, 32)); }
        // panic!();
        self
    }

    /// <summary>
    /// Emits a push <see cref="Instruction"/> with the specified boolean value.
    /// </summary>
    /// <param name="value">The value to be pushed.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitPushBool(&mut self, value:bool) -> &mut ScriptBuilder
    {
        self.Emit(if value { OpCode::PUSH1 } else { OpCode::PUSH0 }, None);
        self
    }

    /// <summary>
    /// Emits a push <see cref="Instruction"/> with the specified data.
    /// </summary>
    /// <param name="data">The data to be pushed.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitPushData(&mut self, data: &[u8]) -> &mut ScriptBuilder
    {
        // if (data == null)
        // throw new ArgumentNullException(nameof(data));
        if data.len() < 0x100
        {
            self.Emit(OpCode.PUSHDATA1);
            self.writer.Write((byte)data.Length);
            self.writer.Write(data);
        } else if data.Length < 0x10000
        {
            self.Emit(OpCode.PUSHDATA2);
            self.writer.Write((ushort)data.Length);
            self.writer.Write(data);
        } else// if (data.Length < 0x100000000L)
        {
            self.Emit(OpCode.PUSHDATA4);
            self.writer.Write(data.Length);
            self.writer.Write(data);
        }
        self
    }

    /// <summary>
    /// Emits a push <see cref="Instruction"/> with the specified <see cref="string"/>.
    /// </summary>
    /// <param name="data">The <see cref="string"/> to be pushed.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitPushString(&mut self, data: &str) -> &mut ScriptBuilder
    {
        self.EmitPush(Utility.StrictUTF8.GetBytes(data));
        self
    }

    /// <summary>
    /// Emits raw script.
    /// </summary>
    /// <param name="script">The raw script to be emitted.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitRaw(&mut self, script:Option<byte>) -> &mut ScriptBuilder
    {
        match script{
            Some(v)=>self.writer.Write(script),
            None=>()
        }
        self
    }

    /// <summary>
    /// Emits an <see cref="Instruction"/> with <see cref="OpCode.SYSCALL"/>.
    /// </summary>
    /// <param name="api">The operand of <see cref="OpCode.SYSCALL"/>.</param>
    /// <returns>A reference to this instance after the emit operation has completed.</returns>
    pub fn EmitSysCall(&mut self, api: u32) -> &mut ScriptBuilder
    {
        self.Emit(OpCode.SYSCALL, api.to_be_bytes());
        self
    }

    /// <summary>
    /// Converts the value of this instance to a byte array.
    /// </summary>
    /// <returns>A byte array contains the script.</returns>
    pub fn ToArray(&mut self)->
    {
    self.writer.Flush();
        self.ms.ToArray()
    }

    fn PadRight(data:&[u8], length:i32) -> &[u8]
    {
    if data.Length >= length{ return data;}
    let mut buffer = new byte[length];
    Buffer.BlockCopy(data, 0, buffer, 0, data.Length);
    return buffer;
    }
}