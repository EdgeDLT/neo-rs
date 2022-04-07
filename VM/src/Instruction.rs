use getset::{CopyGetters, Getters, MutGetters, Setters};

use crate::Memory::Memory;
use crate::OpCode::OpCode;

#[derive(Getters, Setters, MutGetters, CopyGetters, Default,Debug, Clone, Default, Eq, PartialEq)]
pub struct Instruction {
    #[getset(get_copy = "pub",get="pub", set)]
    pub(crate) opcode: OpCode,
    #[getset(get_mut = "pub", set)]
    operand: Memory,
    #[getset(get_mut = "pub", set)]
    operand_size_prefix_table: [i32; 256],
    operand_size_table: [i32; 256],
}


impl Default for Instruction
{
    fn default()->Self
    {
        foreach(FieldInfo field in typeof(OpCode).GetFields(BindingFlags.Public | BindingFlags.Static))
        {
            OperandSizeAttribute?
            attribute = field.GetCustomAttribute < OperandSizeAttribute > ();
            if attribute == null {
                continue;
            }
            int
            index = (int)(OpCode)
            field.GetValue(null)
            !;
            OperandSizePrefixTable[index] = attribute.SizePrefix;
            OperandSizeTable[index] = attribute.Size;
        }
    }
}

/// <summary>
/// Represents instructions in the VM script.
/// </summary>
impl Instruction {
    /// <summary>
    /// Represents the instruction with <see cref="OpCode.RET"/>.
    /// </summary>
    pub fn ret(&self) -> Self {
        Self {
            opcode: OpCode::RET,
            operand: Memory {
                data: vec![],
                effective_len: Default::default(),
                limit: 0,
            },
            operand_size_prefix_table: [i32; 256],
            operand_size_table: [i32; 256],
        }
    }

    // /// <summary>
    // /// The <see cref="VM.OpCode"/> of the instruction.
    // /// </summary>
    //  readonly OpCode OpCode;

    /// <summary>
    /// The operand of the instruction.
    /// </summary>


    /// <summary>
    /// Gets the size of the instruction.
    /// </summary>
    pub fn size(&self) -> i32
    {
        let prefixSize = self.operand_size_prefix_table[self.opcode as usize];

        match prefixSize > 0 {
            true => { 1 + prefixSize + self.operand.len() }
            false => { 1 + self.operand_size_table[self.opcode as usize] }
        }
    }

    /// <summary>
    /// Gets the first operand as <see cref="short"/>.
    /// </summary>
    pub fn token_i16(&self) -> i16
    {
        return BinaryPrimitives.ReadInt16LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="int"/>.
    /// </summary>
    pub fn token_i32(&self) -> i32
    {
        return BinaryPrimitives.ReadInt32LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the second operand as <see cref="int"/>.
    /// </summary>
    pub fn token_i32_1(&self) -> i32
    {
        return BinaryPrimitives.ReadInt32LittleEndian(Operand.Span[4..]);
    }

    /// <summary>
    /// Gets the first operand as <see cref="i8"/>.
    /// </summary>
    pub fn token_i8(&self) -> i8
    {
        self.operand.Span[0] as i8
    }

    /// <summary>
    /// Gets the second operand as <see cref="i8"/>.
    /// </summary>
    pub fn token_i8_1(&self) -> i8
    {
        self.operand.Span[1] as i8
    }

    /// <summary>
    /// Gets the operand as <see cref="string"/>.
    /// </summary>
    pub fn token_string(&self) -> String
    {
        return Encoding.ASCII.GetString(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="ushort"/>.
    /// </summary>
    pub fn token_u16(&self) -> u16
    {
        return BinaryPrimitives.ReadUInt16LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="uint"/>.
    /// </summary>
    pub fn token_u32(&self) -> u32
    {
        return BinaryPrimitives.ReadUInt32LittleEndian(Operand.Span);
    }

    /// <summary>
    /// Gets the first operand as <see cref="byte"/>.
    /// </summary>
    pub fn token_u8(&self) -> u8
    {
        self.operand.Span[0]
    }

    /// <summary>
    /// Gets the second operand as <see cref="byte"/>.
    /// </summary>
    pub fn token_u8_1(&self) -> u8
    {
        self.operand.Span[1]
    }



    pub fn from_opcode(opcode: OpCode) -> Self
    {
        Self {
            opcode,
            operand: Memory {
                data: vec![],
                effective_len: Default::default(),
                limit: 0,
            },
            operand_size_prefix_table: [i32; 256],
            operand_size_table: [i32; 256],
        }
    }

    pub fn from_script(script: &[u8], mut ip: i32)->Self
    {
        ip = ip + 1;
        let instruction = Self::from_OpCode(script[ip], ());
        let operand_size_prefix = instruction.OperandSizePrefixTable[instruction.OpCode.0];
        let mut operandSize = 0;
        match operand_size_prefix
        {
            0 => operandSize = OperandSizeTable[OpCode.0],
            1 => operandSize = script[ip],
            2 => operandSize = BitConverter.ToUInt16(script, ip),
            4 => operandSize = BitConverter.ToInt32(script, ip)
        }
        if operandSize > 0
        {
            ip = ip+ operand_size_prefix;
            if ip + operandSize > script.Length {
                panic!();
            }
            Operand =  Memory{ data: Vec::from(script.slice(ip as usize, operandSize as usize)),
                effective_len: Default::default(),
                limit: operandSize as usize
            };
        }

        instruction
    }
}
