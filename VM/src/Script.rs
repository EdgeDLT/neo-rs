use std::collections::HashMap;

use crate::Instruction::Instruction;
use crate::OpCode::{OpCode, toOpCode};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Script {
    _value: [u8],
    strictMode: bool,
    _instructions: HashMap<usize, Instruction>,
}

/// <summary>
/// Represents the script executed in the VM.
/// </summary>
// [DebuggerDisplay("Length={Length}")]
impl Script
{
    /// <summary>
    /// The length of the script.
    /// </summary>
    pub fn Length(&self) -> usize
    {
        self._value.len() as usize
    }

    /// <summary>
    /// Gets the <see cref="OpCode"/> at the specified index.
    /// </summary>
    /// <param name="index">The index to locate.</param>
    /// <returns>The <see cref="OpCode"/> at the specified index.</returns>
    pub fn At(&self, index: usize) -> OpCode
    {
        toOpCode(self._value[index]).unwrap()
    }

    /// <summary>
    /// Initializes a new instance of the <see cref="Script"/> class.
    /// </summary>
    /// <param name="script">The bytecodes of the script.</param>
    // fn Script(byte[] script): this(script, false)
    // {}

    /// <summary>
    /// Initializes a new instance of the <see cref="Script"/> class.
    /// </summary>
    /// <param name="script">The bytecodes of the script.</param>
    /// <param name="strictMode">
    /// Indicates whether strict mode is enabled.
    /// In strict mode, the script will be checked, but the loading speed will be slower.
    /// </param>
    /// <exception cref="BadScriptException">In strict mode, the script was found to contain bad instructions.</exception>
    pub fn from_bytes(&mut self, script: &[u8], trictMode: bool)
    {
        self._value = *script.clone();
        if strictMode
        {
            for mut ip in 0..self.script.len() { ip += GetInstruction(ip).Size; }

            for (ip, instruction) in self._instructions
            {
                match instruction.OpCode {
                    OpCode::JMP |
                    OpCode::JMPIF |
                    OpCode::JMPIFNOT |
                    OpCode::JMPEQ |
                    OpCode::JMPNE |
                    OpCode::JMPGT |
                    OpCode::JMPGE |
                    OpCode::JMPLT |
                    OpCode::JMPLE |
                    OpCode::CALL |
                    OpCode::ENDTRY => {
                        if !self._instructions.contains_key(checked(ip + instruction.TokenI8)) {
                            panic!();
                            //      throw
                            // new
                            // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        }
                        break;
                    }
                    OpCode::PUSHA |
                    OpCode::JMP_L |
                    OpCode::JMPIF_L |
                    OpCode::JMPIFNOT_L |
                    OpCode::JMPEQ_L |
                    OpCode::JMPNE_L |
                    OpCode::JMPGT_L |
                    OpCode::JMPGE_L |
                    OpCode::JMPLT_L |
                    OpCode::JMPLE_L |
                    OpCode::CALL_L |
                    OpCode::ENDTRY_L => {
                        if !self._instructions.contains_key(checked(ip + instruction.TokenI32)) { panic!(); }
                        // throw
                        // new
                        // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        break;
                    }
                    OpCode::TRY => {
                        if !self._instructions.contains_key(checked(ip + instruction.ØTokenI8)) { panic!(); }
                        // throw
                        // new
                        // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        if !self._instructions.contains_key(checked(ip + instruction.TokenI8_1)) { panic!(); }
                        // throw
                        // new
                        // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        break;
                    }
                    OpCode::TRY_L => {
                        if !self._instructions.ContainsKey(checked(ip + instruction.TokenI32)) { panic!(); }
                        // throw
                        // new
                        // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        if !self._instructions.ContainsKey(checked(ip + instruction.TokenI32_1)) { panic!(); }
                        // throw
                        // new
                        // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        break;
                    }
                    OpCode::NEWARRAY_T |
                    OpCode::ISTYPE |
                    OpCode::CONVERT => {
                        let Type: StackItemType = instruction.TokenU8() as StackItemType;
                        if !Enum.IsDefined(typeof(StackItemType), Type) { panic!(); }
                        // throw
                        // new
                        // BadScriptException();
                        if instruction.OpCode != OpCode.NEWARRAY_T && Type == StackItemType.Any { panic!(); }
                        // throw
                        // new
                        // BadScriptException($ "ip: {ip}, opcode: {instruction.OpCode}");
                        break;
                    }
                    _ => { unreachable!() }
                }
            }
        }
        this.strictMode = strictMode;
    }

    /// <summary>
    /// Get the <see cref="Instruction"/> at the specified position.
    /// </summary>
    /// <param name="ip">The position to get the <see cref="Instruction"/>.</param>
    /// <returns>The <see cref="Instruction"/> at the specified position.</returns>
    /// <exception cref="ArgumentException">In strict mode, the <see cref="Instruction"/> was not found at the specified position.</exception>
    pub fn GetInstruction(&self, ip: usize) -> Instruction
    {
        if ip >= self.Length() { return Instruction.RET; }

        if !self._instructions.TryGetValue(ip, out Instruction? instruction)
        {
            if self.strictMode { panic!(); }
            // throw new ArgumentException( $ "ip not found with strict mode", nameof(ip));
            instruction =
            Instruction{_value, ip};
            _instructions.Add(ip, instruction);
        }
        return instruction;
    }

    // public static implicit operator byte[](Script script) => script._value;
    // public static implicit operator Script(byte[] script) => new(script);
}