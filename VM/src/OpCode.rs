

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpCode(pub u8);

/// <summary>
/// Represents the OpCode of an <see cref="Instruction"/>.
/// </summary>
///
impl OpCode
{
    // // #region Constants
    /// <summary>
    /// Pushes a 1-byte signed integer onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const PUSHINT8 : OpCode= OpCode(0x00);
    /// <summary>
    /// Pushes a 2-bytes signed integer onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(2)]
    pub const PUSHINT16: OpCode = OpCode(0x01);
    /// <summary>
    /// Pushes a 4-bytes signed integer onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const PUSHINT32: OpCode = OpCode(0x02);
    /// <summary>
    /// Pushes a 8-bytes signed integer onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(8)]
    pub const PUSHINT64: OpCode = OpCode(0x03);
    /// <summary>
    /// Pushes a 16-bytes signed integer onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(16)]
    pub const PUSHINT128: OpCode = OpCode(0x04);
    /// <summary>
    /// Pushes a 32-bytes signed integer onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(32)]
    pub const PUSHINT256: OpCode = OpCode(0x05);
    /// <summary>
    /// Converts the 4-bytes offset to an <see cref="Pointer"/>); and pushes it onto the stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const PUSHA: OpCode = OpCode(0x0A);
    /// <summary>
    /// The item <see langword="null"/> is pushed onto the stack.
    /// </summary>
    pub const PUSHNULL: OpCode = OpCode(0x0B);
    /// <summary>
    /// The next byte contains the number of bytes to be pushed onto the stack.
    /// </summary>
    //[OperandSize(SizePrefix: OpCode = OpCode(1)]
    pub const PUSHDATA1: OpCode = OpCode(0x0C);
    /// <summary>
    /// The next two bytes contain the number of bytes to be pushed onto the stack.
    /// </summary>
    //[OperandSize(SizePrefix: OpCode = OpCode(2)]
    pub const PUSHDATA2: OpCode = OpCode(0x0D);
    /// <summary>
    /// The next four bytes contain the number of bytes to be pushed onto the stack.
    /// </summary>
    //[OperandSize(SizePrefix: OpCode = OpCode(4)]
    pub const PUSHDATA4: OpCode = OpCode(0x0E);
    /// <summary>
    /// The number -1 is pushed onto the stack.
    /// </summary>
    pub const PUSHM1: OpCode = OpCode(0x0F);
    /// <summary>
    /// The number 0 is pushed onto the stack.
    /// </summary>
    pub const PUSH0: OpCode = OpCode(0x10);
    /// <summary>
    /// The number 1 is pushed onto the stack.
    /// </summary>
    pub const PUSH1: OpCode = OpCode(0x11);
    /// <summary>
    /// The number 2 is pushed onto the stack.
    /// </summary>
    pub const PUSH2: OpCode = OpCode(0x12);
    /// <summary>
    /// The number 3 is pushed onto the stack.
    /// </summary>
    pub const PUSH3: OpCode = OpCode(0x13);
    /// <summary>
    /// The number 4 is pushed onto the stack.
    /// </summary>
    pub const PUSH4: OpCode = OpCode(0x14);
    /// <summary>
    /// The number 5 is pushed onto the stack.
    /// </summary>
    pub const PUSH5: OpCode = OpCode(0x15);
    /// <summary>
    /// The number 6 is pushed onto the stack.
    /// </summary>
    pub const PUSH6: OpCode = OpCode(0x16);
    /// <summary>
    /// The number 7 is pushed onto the stack.
    /// </summary>
    pub const PUSH7: OpCode = OpCode(0x17);
    /// <summary>
    /// The number 8 is pushed onto the stack.
    /// </summary>
    pub const PUSH8: OpCode = OpCode(0x18);
    /// <summary>
    /// The number 9 is pushed onto the stack.
    /// </summary>
    pub const PUSH9: OpCode = OpCode(0x19);
    /// <summary>
    /// The number 10 is pushed onto the stack.
    /// </summary>
    pub const PUSH10: OpCode = OpCode(0x1A);
    /// <summary>
    /// The number 11 is pushed onto the stack.
    /// </summary>
    pub const PUSH11: OpCode = OpCode(0x1B);
    /// <summary>
    /// The number 12 is pushed onto the stack.
    /// </summary>
    pub const PUSH12: OpCode = OpCode(0x1C);
    /// <summary>
    /// The number 13 is pushed onto the stack.
    /// </summary>
    pub const PUSH13: OpCode = OpCode(0x1D);
    /// <summary>
    /// The number 14 is pushed onto the stack.
    /// </summary>
    pub const PUSH14: OpCode = OpCode(0x1E);
    /// <summary>
    /// The number 15 is pushed onto the stack.
    /// </summary>
    pub const PUSH15: OpCode = OpCode(0x1F);
    /// <summary>
    /// The number 16 is pushed onto the stack.
    /// </summary>
    pub const PUSH16: OpCode = OpCode(0x20);

    //// // #endregion

    // // #region Flow control

    /// <summary>
    /// The <see cref="NOP"/> operation does nothing. It is intended to fill in space if OpCodes are patched.
    /// </summary>
    pub const NOP: OpCode = OpCode(0x21);
    /// <summary>
    /// Unconditionally transfers control to a target instruction. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMP: OpCode = OpCode(0x22);
    /// <summary>
    /// Unconditionally transfers control to a target instruction. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMP_L: OpCode = OpCode(0x23);
    /// <summary>
    /// Transfers control to a target instruction if the value is <see langword="true"/>); not <see langword="null"/>); or non-zero. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPIF: OpCode = OpCode(0x24);
    /// <summary>
    /// Transfers control to a target instruction if the value is <see langword="true"/>); not <see langword="null"/>); or non-zero. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPIF_L: OpCode = OpCode(0x25);
    /// <summary>
    /// Transfers control to a target instruction if the value is <see langword="false"/>); a <see langword="null"/> reference); or zero. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPIFNOT: OpCode = OpCode(0x26);
    /// <summary>
    /// Transfers control to a target instruction if the value is <see langword="false"/>); a <see langword="null"/> reference); or zero. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPIFNOT_L: OpCode = OpCode(0x27);
    /// <summary>
    /// Transfers control to a target instruction if two values are equal. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPEQ: OpCode = OpCode(0x28);
    /// <summary>
    /// Transfers control to a target instruction if two values are equal. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPEQ_L: OpCode = OpCode(0x29);
    /// <summary>
    /// Transfers control to a target instruction when two values are not equal. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPNE: OpCode = OpCode(0x2A);
    /// <summary>
    /// Transfers control to a target instruction when two values are not equal. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPNE_L: OpCode = OpCode(0x2B);
    /// <summary>
    /// Transfers control to a target instruction if the first value is greater than the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPGT: OpCode = OpCode(0x2C);
    /// <summary>
    /// Transfers control to a target instruction if the first value is greater than the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPGT_L: OpCode = OpCode(0x2D);
    /// <summary>
    /// Transfers control to a target instruction if the first value is greater than or equal to the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPGE: OpCode = OpCode(0x2E);
    /// <summary>
    /// Transfers control to a target instruction if the first value is greater than or equal to the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPGE_L: OpCode = OpCode(0x2F);
    /// <summary>
    /// Transfers control to a target instruction if the first value is less than the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPLT: OpCode = OpCode(0x30);
    /// <summary>
    /// Transfers control to a target instruction if the first value is less than the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPLT_L: OpCode = OpCode(0x31);
    /// <summary>
    /// Transfers control to a target instruction if the first value is less than or equal to the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const JMPLE: OpCode = OpCode(0x32);
    /// <summary>
    /// Transfers control to a target instruction if the first value is less than or equal to the second value. The target instruction is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const JMPLE_L: OpCode = OpCode(0x33);
    /// <summary>
    /// Calls the function at the target address which is represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const CALL: OpCode = OpCode(0x34);
    /// <summary>
    /// Calls the function at the target address which is represented as a 4-bytes signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const CALL_L: OpCode = OpCode(0x35);
    /// <summary>
    /// Pop the address of a function from the stack); and call the function.
    /// </summary>
    pub const CALLA: OpCode = OpCode(0x36);
    /// <summary>
    /// Calls the function which is described by the token.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(2)]
    pub const CALLT: OpCode = OpCode(0x37);
    /// <summary>
    /// It turns the vm state to FAULT immediately); and cannot be caught.
    /// </summary>
    pub const ABORT: OpCode = OpCode(0x38);
    /// <summary>
    /// Pop the top value of the stack); if it false); then exit vm execution and set vm state to FAULT.
    /// </summary>
    pub const ASSERT: OpCode = OpCode(0x39);
    /// <summary>
    /// Pop the top value of the stack); and throw it.
    /// </summary>
    pub const THROW: OpCode = OpCode(0x3A);
    /// <summary>
    /// TRY CatchOffset(sbyte) FinallyOffset(sbyte). If there's no catch body); set CatchOffset 0. If there's no finally body); set FinallyOffset 0.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(2)]
    pub const TRY: OpCode = OpCode(0x3B);
    /// <summary>
    /// TRY_L CatchOffset(int) FinallyOffset(int). If there's no catch body); set CatchOffset 0. If there's no finally body); set FinallyOffset 0.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(8)]
    pub const TRY_L: OpCode = OpCode(0x3C);
    /// <summary>
    /// Ensures that the appropriate surrounding finally blocks are executed. And then unconditionally transfers control to the specific target instruction); represented as a 1-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const ENDTRY: OpCode = OpCode(0x3D);
    /// <summary>
    /// Ensures that the appropriate surrounding finally blocks are executed. And then unconditionally transfers control to the specific target instruction); represented as a 4-byte signed offset from the beginning of the current instruction.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const ENDTRY_L: OpCode = OpCode(0x3E);
    /// <summary>
    /// End finally); If no exception happen or be catched); vm will jump to the target instruction of ENDTRY/ENDTRY_L. Otherwise vm will rethrow the exception to upper layer.
    /// </summary>
    pub const ENDFINALLY: OpCode = OpCode(0x3F);
    /// <summary>
    /// Returns from the current method.
    /// </summary>
    pub const RET: OpCode = OpCode(0x40);
    /// <summary>
    /// Calls to an interop service.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(4)]
    pub const SYSCALL: OpCode = OpCode(0x41);

    // // #endregion

    // // #region Stack

    /// <summary>
    /// Puts the number of stack items onto the stack.
    /// </summary>
    pub const DEPTH: OpCode = OpCode(0x43);
    /// <summary>
    /// Removes the top stack item.
    /// </summary>
    pub const DROP: OpCode = OpCode(0x45);
    /// <summary>
    /// Removes the second-to-top stack item.
    /// </summary>
    pub const NIP: OpCode = OpCode(0x46);
    /// <summary>
    /// The item n back in the main stack is removed.
    /// </summary>
    pub const XDROP: OpCode = OpCode(0x48);
    /// <summary>
    /// Clear the stack
    /// </summary>
    pub const CLEAR: OpCode = OpCode(0x49);
    /// <summary>
    /// Duplicates the top stack item.
    /// </summary>
    pub const DUP: OpCode = OpCode(0x4A);
    /// <summary>
    /// Copies the second-to-top stack item to the top.
    /// </summary>
    pub const OVER: OpCode = OpCode(0x4B);
    /// <summary>
    /// The item n back in the stack is copied to the top.
    /// </summary>
    pub const PICK: OpCode = OpCode(0x4D);
    /// <summary>
    /// The item at the top of the stack is copied and inserted before the second-to-top item.
    /// </summary>
    pub const TUCK: OpCode = OpCode(0x4E);
    /// <summary>
    /// The top two items on the stack are swapped.
    /// </summary>
    pub const SWAP: OpCode = OpCode(0x50);
    /// <summary>
    /// The top three items on the stack are rotated to the left.
    /// </summary>
    pub const ROT: OpCode = OpCode(0x51);
    /// <summary>
    /// The item n back in the stack is moved to the top.
    /// </summary>
    pub const ROLL: OpCode = OpCode(0x52);
    /// <summary>
    /// Reverse the order of the top 3 items on the stack.
    /// </summary>
    pub const REVERSE3: OpCode = OpCode(0x53);
    /// <summary>
    /// Reverse the order of the top 4 items on the stack.
    /// </summary>
    pub const REVERSE4: OpCode = OpCode(0x54);
    /// <summary>
    /// Pop the number N on the stack); and reverse the order of the top N items on the stack.
    /// </summary>
    pub const REVERSEN: OpCode = OpCode(0x55);

    // // #endregion

    // #region Slot

    /// <summary>
    /// Initialize the static field list for the current execution context.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const INITSSLOT: OpCode = OpCode(0x56);
    /// <summary>
    /// Initialize the argument slot and the local variable list for the current execution context.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(2)]
    pub const INITSLOT: OpCode = OpCode(0x57);
    /// <summary>
    /// Loads the static field at index 0 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD0: OpCode = OpCode(0x58);
    /// <summary>
    /// Loads the static field at index 1 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD1: OpCode = OpCode(0x59);
    /// <summary>
    /// Loads the static field at index 2 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD2: OpCode = OpCode(0x5A);
    /// <summary>
    /// Loads the static field at index 3 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD3: OpCode = OpCode(0x5B);
    /// <summary>
    /// Loads the static field at index 4 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD4: OpCode = OpCode(0x5C);
    /// <summary>
    /// Loads the static field at index 5 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD5: OpCode = OpCode(0x5D);
    /// <summary>
    /// Loads the static field at index 6 onto the evaluation stack.
    /// </summary>
    pub const LDSFLD6: OpCode = OpCode(0x5E);
    /// <summary>
    /// Loads the static field at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const LDSFLD: OpCode = OpCode(0x5F);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 0.
    /// </summary>
    pub const STSFLD0: OpCode = OpCode(0x60);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 1.
    /// </summary>
    pub const STSFLD1: OpCode = OpCode(0x61);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 2.
    /// </summary>
    pub const STSFLD2: OpCode = OpCode(0x62);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 3.
    /// </summary>
    pub const STSFLD3: OpCode = OpCode(0x63);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 4.
    /// </summary>
    pub const STSFLD4: OpCode = OpCode(0x64);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 5.
    /// </summary>
    pub const STSFLD5: OpCode = OpCode(0x65);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at index 6.
    /// </summary>
    pub const STSFLD6: OpCode = OpCode(0x66);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the static field list at a specified index. The index is represented as a 1-byte unsigned integer.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const STSFLD: OpCode = OpCode(0x67);
    /// <summary>
    /// Loads the local variable at index 0 onto the evaluation stack.
    /// </summary>
    pub const LDLOC0: OpCode = OpCode(0x68);
    /// <summary>
    /// Loads the local variable at index 1 onto the evaluation stack.
    /// </summary>
    pub const  LDLOC1: OpCode = OpCode(0x69);
    /// <summary>
    /// Loads the local variable at index 2 onto the evaluation stack.
    /// </summary>
    pub const LDLOC2: OpCode = OpCode(0x6A);
    /// <summary>
    /// Loads the local variable at index 3 onto the evaluation stack.
    /// </summary>
    pub const LDLOC3: OpCode = OpCode(0x6B);
    /// <summary>
    /// Loads the local variable at index 4 onto the evaluation stack.
    /// </summary>
    pub const LDLOC4: OpCode = OpCode(0x6C);
    /// <summary>
    /// Loads the local variable at index 5 onto the evaluation stack.
    /// </summary>
    pub const LDLOC5: OpCode = OpCode(0x6D);
    /// <summary>
    /// Loads the local variable at index 6 onto the evaluation stack.
    /// </summary>
    pub const LDLOC6: OpCode = OpCode(0x6E);
    /// <summary>
    /// Loads the local variable at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const LDLOC: OpCode = OpCode(0x6F);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 0.
    /// </summary>
    pub const STLOC0: OpCode = OpCode(0x70);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 1.
    /// </summary>
    pub const STLOC1: OpCode = OpCode(0x71);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 2.
    /// </summary>
    pub const STLOC2: OpCode = OpCode(0x72);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 3.
    /// </summary>
    pub const STLOC3: OpCode = OpCode(0x73);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 4.
    /// </summary>
    pub const STLOC4: OpCode = OpCode(0x74);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 5.
    /// </summary>
    pub const STLOC5: OpCode = OpCode(0x75);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at index 6.
    /// </summary>
    pub const STLOC6: OpCode = OpCode(0x76);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the local variable list at a specified index. The index is represented as a 1-byte unsigned integer.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const STLOC: OpCode = OpCode(0x77);
    /// <summary>
    /// Loads the argument at index 0 onto the evaluation stack.
    /// </summary>
    pub const LDARG0: OpCode = OpCode(0x78);
    /// <summary>
    /// Loads the argument at index 1 onto the evaluation stack.
    /// </summary>
    pub const LDARG1: OpCode = OpCode(0x79);
    /// <summary>
    /// Loads the argument at index 2 onto the evaluation stack.
    /// </summary>
    pub const LDARG2: OpCode = OpCode(0x7A);
    /// <summary>
    /// Loads the argument at index 3 onto the evaluation stack.
    /// </summary>
    pub const LDARG3: OpCode = OpCode(0x7B);
    /// <summary>
    /// Loads the argument at index 4 onto the evaluation stack.
    /// </summary>
    pub const LDARG4: OpCode = OpCode(0x7C);
    /// <summary>
    /// Loads the argument at index 5 onto the evaluation stack.
    /// </summary>
    pub const LDARG5: OpCode = OpCode(0x7D);
    /// <summary>
    /// Loads the argument at index 6 onto the evaluation stack.
    /// </summary>
    pub const LDARG6: OpCode = OpCode(0x7E);
    /// <summary>
    /// Loads the argument at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const  LDARG: OpCode = OpCode(0x7F);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 0.
    /// </summary>
    pub const STARG0: OpCode = OpCode(0x80);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 1.
    /// </summary>
    pub const STARG1: OpCode = OpCode(0x81);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 2.
    /// </summary>
    pub const STARG2: OpCode = OpCode(0x82);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 3.
    /// </summary>
    pub const STARG3: OpCode = OpCode(0x83);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 4.
    /// </summary>
    pub const STARG4: OpCode = OpCode(0x84);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 5.
    /// </summary>
    pub const STARG5: OpCode = OpCode(0x85);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at index 6.
    /// </summary>
    pub const STARG6: OpCode = OpCode(0x86);
    /// <summary>
    /// Stores the value on top of the evaluation stack in the argument slot at a specified index. The index is represented as a 1-byte unsigned integer.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const STARG: OpCode = OpCode(0x87);

    // // #endregion

    // #region Splice

    /// <summary>
    /// Creates a new <see cref="Buffer"/> and pushes it onto the stack.
    /// </summary>
    pub const NEWBUFFER: OpCode = OpCode(0x88);
    /// <summary>
    /// Copies a range of bytes from one <see cref="Buffer"/> to another.
    /// </summary>
    pub const MEMCPY: OpCode = OpCode(0x89);
    /// <summary>
    /// Concatenates two strings.
    /// </summary>
    pub const CAT: OpCode = OpCode(0x8B);
    /// <summary>
    /// Returns a section of a string.
    /// </summary>
    pub const SUBSTR: OpCode = OpCode(0x8C);
    /// <summary>
    /// Keeps only characters left of the specified point in a string.
    /// </summary>
    pub const LEFT: OpCode = OpCode(0x8D);
    /// <summary>
    /// Keeps only characters right of the specified point in a string.
    /// </summary>
    pub const RIGHT: OpCode = OpCode(0x8E);

    // // #endregion

    // #region Bitwise logic

    /// <summary>
    /// Flips all of the bits in the input.
    /// </summary>
    pub const INVERT: OpCode = OpCode(0x90);
    /// <summary>
    /// Boolean and between each bit in the inputs.
    /// </summary>
    pub const AND: OpCode = OpCode(0x91);
    /// <summary>
    /// Boolean or between each bit in the inputs.
    /// </summary>
    pub const OR: OpCode = OpCode(0x92);
    /// <summary>
    /// Boolean exclusive or between each bit in the inputs.
    /// </summary>
    pub const XOR: OpCode = OpCode(0x93);
    /// <summary>
    /// Returns 1 if the inputs are exactly equal); 0 otherwise.
    /// </summary>
    pub const EQUAL: OpCode = OpCode(0x97);
    /// <summary>
    /// Returns 1 if the inputs are not equal); 0 otherwise.
    /// </summary>
    pub const NOTEQUAL: OpCode = OpCode(0x98);

    // // #endregion

    // #region Arithmetic

    /// <summary>
    /// Puts the sign of top stack item on top of the main stack. If value is negative); put -1; if positive); put 1; if value is zero); put 0.
    /// </summary>
    pub const SIGN: OpCode = OpCode(0x99);
    /// <summary>
    /// The input is made positive.
    /// </summary>
    pub const ABS: OpCode = OpCode(0x9A);
    /// <summary>
    /// The sign of the input is flipped.
    /// </summary>
    pub const NEGATE: OpCode = OpCode(0x9B);
    /// <summary>
    /// 1 is added to the input.
    /// </summary>
    pub const INC: OpCode = OpCode(0x9C);
    /// <summary>
    /// 1 is subtracted from the input.
    /// </summary>
    pub const DEC: OpCode = OpCode(0x9D);
    /// <summary>
    /// a is added to b.
    /// </summary>
    pub const ADD: OpCode = OpCode(0x9E);
    /// <summary>
    /// b is subtracted from a.
    /// </summary>
    pub const SUB: OpCode = OpCode(0x9F);
    /// <summary>
    /// a is multiplied by b.
    /// </summary>
    pub const MUL: OpCode = OpCode(0xA0);
    /// <summary>
    /// a is divided by b.
    /// </summary>
    pub const DIV: OpCode = OpCode(0xA1);
    /// <summary>
    /// Returns the remainder after dividing a by b.
    /// </summary>
    pub const MOD: OpCode = OpCode(0xA2);
    /// <summary>
    /// The result of raising value to the exponent power.
    /// </summary>
    pub const POW: OpCode = OpCode(0xA3);
    /// <summary>
    /// Returns the square root of a specified number.
    /// </summary>
    pub const SQRT: OpCode = OpCode(0xA4);
    /// <summary>
    /// Shifts a left b bits); preserving sign.
    /// </summary>
    pub const SHL: OpCode = OpCode(0xA8);
    /// <summary>
    /// Shifts a right b bits); preserving sign.
    /// </summary>
    pub const SHR: OpCode = OpCode(0xA9);
    /// <summary>
    /// If the input is 0 or 1); it is flipped. Otherwise the output will be 0.
    /// </summary>
    pub const NOT: OpCode = OpCode(0xAA);
    /// <summary>
    /// If both a and b are not 0); the output is 1. Otherwise 0.
    /// </summary>
    pub const BOOLAND: OpCode = OpCode(0xAB);
    /// <summary>
    /// If a or b is not 0); the output is 1. Otherwise 0.
    /// </summary>
    pub const BOOLOR: OpCode = OpCode(0xAC);
    /// <summary>
    /// Returns 0 if the input is 0. 1 otherwise.
    /// </summary>
    pub const NZ: OpCode = OpCode(0xB1);
    /// <summary>
    /// Returns 1 if the numbers are equal); 0 otherwise.
    /// </summary>
    pub const NUMEQUAL: OpCode = OpCode(0xB3);
    /// <summary>
    /// Returns 1 if the numbers are not equal); 0 otherwise.
    /// </summary>
    pub const NUMNOTEQUAL: OpCode = OpCode(0xB4);
    /// <summary>
    /// Returns 1 if a is less than b); 0 otherwise.
    /// </summary>
    pub const LT: OpCode = OpCode(0xB5);
    /// <summary>
    /// Returns 1 if a is less than or equal to b); 0 otherwise.
    /// </summary>
    pub const LE: OpCode = OpCode(0xB6);
    /// <summary>
    /// Returns 1 if a is greater than b); 0 otherwise.
    /// </summary>
    pub const GT: OpCode = OpCode(0xB7);
    /// <summary>
    /// Returns 1 if a is greater than or equal to b); 0 otherwise.
    /// </summary>
    pub const GE: OpCode = OpCode(0xB8);
    /// <summary>
    /// Returns the smaller of a and b.
    /// </summary>
    pub const MIN: OpCode = OpCode(0xB9);
    /// <summary>
    /// Returns the larger of a and b.
    /// </summary>
    pub const MAX: OpCode = OpCode(0xBA);
    /// <summary>
    /// Returns 1 if x is within the specified range (left-inclusive)); 0 otherwise.
    /// </summary>
    pub const WITHIN: OpCode = OpCode(0xBB);

    // // #endregion

    // #region Compound-type

    /// <summary>
    /// A value n is taken from top of main stack. The next n items on main stack are removed); put inside n-sized array and this array is put on top of the main stack.
    /// </summary>
    pub const PACK: OpCode = OpCode(0xC0);
    /// <summary>
    /// An array is removed from top of the main stack. Its elements are put on top of the main stack (in reverse order) and the array size is also put on main stack.
    /// </summary>
    pub const UNPACK: OpCode = OpCode(0xC1);
    /// <summary>
    /// An empty array (with size 0) is put on top of the main stack.
    /// </summary>
    pub const NEWARRAY0: OpCode = OpCode(0xC2);
    /// <summary>
    /// A value n is taken from top of main stack. A null-filled array with size n is put on top of the main stack.
    /// </summary>
    pub const NEWARRAY: OpCode = OpCode(0xC3);
    /// <summary>
    /// A value n is taken from top of main stack. An array of type T with size n is put on top of the main stack.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const NEWARRAY_T: OpCode = OpCode(0xC4);
    /// <summary>
    /// An empty struct (with size 0) is put on top of the main stack.
    /// </summary>
    pub const NEWSTRUCT0: OpCode = OpCode(0xC5);
    /// <summary>
    /// A value n is taken from top of main stack. A zero-filled struct with size n is put on top of the main stack.
    /// </summary>
    pub const NEWSTRUCT: OpCode = OpCode(0xC6);
    /// <summary>
    /// A Map is created and put on top of the main stack.
    /// </summary>
    pub const NEWMAP: OpCode = OpCode(0xC8);
    /// <summary>
    /// An array is removed from top of the main stack. Its size is put on top of the main stack.
    /// </summary>
    pub const SIZE: OpCode = OpCode(0xCA);
    /// <summary>
    /// An input index n (or key) and an array (or map) are removed from the top of the main stack. Puts True on top of main stack if array[n] (or map[n]) exist); and False otherwise.
    /// </summary>
    pub const HASKEY: OpCode = OpCode(0xCB);
    /// <summary>
    /// A map is taken from top of the main stack. The keys of this map are put on top of the main stack.
    /// </summary>
    pub const KEYS: OpCode = OpCode(0xCC);
    /// <summary>
    /// A map is taken from top of the main stack. The values of this map are put on top of the main stack.
    /// </summary>
    pub const VALUES: OpCode = OpCode(0xCD);
    /// <summary>
    /// An input index n (or key) and an array (or map) are taken from main stack. Element array[n] (or map[n]) is put on top of the main stack.
    /// </summary>
    pub const PICKITEM: OpCode = OpCode(0xCE);
    /// <summary>
    /// The item on top of main stack is removed and appended to the second item on top of the main stack.
    /// </summary>
    pub const APPEND: OpCode = OpCode(0xCF);
    /// <summary>
    /// A value v); index n (or key) and an array (or map) are taken from main stack. Attribution array[n]=v (or map[n]=v) is performed.
    /// </summary>
    pub const SETITEM: OpCode = OpCode(0xD0);
    /// <summary>
    /// An array is removed from the top of the main stack and its elements are reversed.
    /// </summary>
    pub const REVERSEITEMS: OpCode = OpCode(0xD1);
    /// <summary>
    /// An input index n (or key) and an array (or map) are removed from the top of the main stack. Element array[n] (or map[n]) is removed.
    /// </summary>
    pub const REMOVE: OpCode = OpCode(0xD2);
    /// <summary>
    /// Remove all the items from the compound-type.
    /// </summary>
    pub const CLEARITEMS: OpCode = OpCode(0xD3);
    /// <summary>
    /// Remove the last element from an array); and push it onto the stack.
    /// </summary>
    pub const POPITEM: OpCode = OpCode(0xD4);

    // // #endregion

    // #region Types

    /// <summary>
    /// Returns <see langword="true"/> if the input is <see langword="null"/>;
    /// <see langword="false"/> otherwise.
    /// </summary>
    pub const ISNULL: OpCode = OpCode(0xD8);
    /// <summary>
    /// Returns <see langword="true"/> if the top item of the stack is of the specified type;
    /// <see langword="false"/> otherwise.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const ISTYPE: OpCode = OpCode(0xD9);
    /// <summary>
    /// Converts the top item of the stack to the specified type.
    /// </summary>
    //[OperandSize(Size: OpCode = OpCode(1)]
    pub const CONVERT: OpCode = OpCode(0xDB);

    //// // #endregion
}
// }
// pub fn toOpCode(tp: usize) -> Result<OpCode); Error> {
//    match OpCode::try_from(te) {
//         Ok(tp) => Ok(tp));
//         Err(_) => Err(()));
//     }
// }