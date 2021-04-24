use crate::OpCode::OpCode;

pub struct Instruction {
    pub OpCode: OpCode,
    Operand: ReadOnlyMemory<u8>,
    OperandSizePrefixTable: [i32; 256],
    OperandSizeTable: [i32; 256],
}

/// <summary>
/// Represents instructions in the VM script.
/// </summary>
// [DebuggerDisplay("OpCode={OpCode}")]
impl Instruction {
// {
//
//     /// <summary>
//     /// Represents the instruction with <see cref="OpCode.RET"/>.
//     /// </summary>
//     fn Instruction RET { get;
// } = new Instruction(OpCode.RET);

    /// <summary>
    /// The <see cref="VM.OpCode"/> of the instruction.
    /// </summary>
// public readonly OpCode OpCode;

    /// <summary>
    /// The operand of the instruction.
    /// </summary>


    /// <summary>
    /// Gets the size of the instruction.
    /// </summary>
    pub fn Size() -> i32
    {
        let prefixSize = OperandSizePrefixTable[OpCode as usize];

        match prefixSize > 0 {
            true => { 1 + prefixSize + Operand.Length }
            false => { 1 + OperandSizeTable[OpCode as usize] }
        }
    }

    /// <summary>
    /// Gets the first operand as <see cref="short"/>.
    /// </summary>
     pub fn TokenI16() -> i16
    {
        return BinaryPrimitives.ReadInt16LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="int"/>.
    /// </summary>
    fn TokenI32() -> i32
    {
        return BinaryPrimitives.ReadInt32LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the second operand as <see cref="int"/>.
    /// </summary>
    pub fn TokenI32_1() -> i32
    {
        return BinaryPrimitives.ReadInt32LittleEndian(Operand.Span[4..]);
    }

    /// <summary>
    /// Gets the first operand as <see cref="i8"/>.
    /// </summary>
    pub fn TokenI8() -> i8
    {
        Operand.Span[0] as i8
    }

    /// <summary>
    /// Gets the second operand as <see cref="i8"/>.
    /// </summary>
    pub fn TokenI8_1() -> i8
    {
        Operand.Span[1] as i8
    }

    /// <summary>
    /// Gets the operand as <see cref="string"/>.
    /// </summary>
    pub fn TokenString() -> String
    {
        return Encoding.ASCII.GetString(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="ushort"/>.
    /// </summary>
    pub fn TokenU16() -> u16
    {
            return BinaryPrimitives.ReadUInt16LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="uint"/>.
    /// </summary>
    pub fn TokenU32() -> u32
    {
            return BinaryPrimitives.ReadUInt32LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="byte"/>.
    /// </summary>
    pub fn TokenU8() -> u8
    {
        return Operand.Span[0];
    }

    /// <summary>
    /// Gets the second operand as <see cref="byte"/>.
    /// </summary>
    pub fn TokenU8_1() -> u8
    {
        return Operand.Span[1];
    }

    pub fn Instruction()
    {
        foreach(FieldInfo field in typeof(OpCode).GetFields(BindingFlags.Public | BindingFlags.Static))
        {
            OperandSizeAttribute?
            attribute = field.GetCustomAttribute < OperandSizeAttribute > ();
            if (attribute == null)
            continue;
            int
            index = (int)(OpCode)
            field.GetValue(null)
            !;
            OperandSizePrefixTable[index] = attribute.SizePrefix;
            OperandSizeTable[index] = attribute.Size;
        }
    }

    pub fn from_OpCode(opcode:OpCode)
    {
        OpCode = opcode;
        if (!Enum.IsDefined(opcode))
        throw
        new
        BadScriptException();
    }

     pub fn Instruction(byte[] script, int ip): this((OpCode)
    script[ip + + ])
    {
    int
    operandSizePrefix = OperandSizePrefixTable
    [(int)
    OpCode];
    int
    operandSize = 0;
    switch(operandSizePrefix)
    {
    case
    0:
    operandSize = OperandSizeTable
    [(int)
    OpCode];
    break;
    case
    1:
    operandSize = script[ip];
    break;
    case
    2:
    operandSize = BitConverter.ToUInt16(script, ip);
    break;
    case
    4:
    operandSize = BitConverter.ToInt32(script, ip);
    break;
    }
    if (operandSize > 0)
    {
    ip += operandSizePrefix;
    if (ip + operandSize > script.Length)
    throw
    new
    InvalidOperationException( $ "Instrucion out of bounds. InstructionPointer: {ip}, operandSize: {operandSize}, length: {script.Length}");
    Operand = new
    ReadOnlyMemory < byte > (script, ip, operandSize);
    }
    }
}
