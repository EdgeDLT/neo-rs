use std::ptr::null;

use crate::EvaluationStack::EvaluationStack;
use crate::ExecutionContext::ExecutionContext;
use crate::ExecutionEngineLimits::ExecutionEngineLimits;
use crate::OpCode::OpCode;
use crate::ReferenceCounter::ReferenceCounter;
use crate::Script::Script;
use crate::VMState::VMState;
use crate::ExceptionHandlingContext::ExceptionHandlingContext;
use std::collections::VecDeque;

pub struct ExecutionEngine {
    state: VMState,
    // = VMState.BREAK;
    isJumping: bool,// = false;

    /// <summary>
    /// Restrictions on the VM.
    /// </summary>
    Limits: ExecutionEngineLimits, //{get; }

    /// <summary>
    /// Used for reference counting of objects in the VM.
    /// </summary>
    ReferenceCounter: ReferenceCounter,// { get; }

    /// <summary>
    /// The invocation stack of the VM.
    /// </summary>
    InvocationStack: VecDeque<ExecutionContext>,

    /// <summary>
    /// The top frame of the invocation stack.
    /// </summary>
    CurrentContext: Option<ExecutionContext>,
    /// <summary>
    /// The bottom frame of the invocation stack.
    /// </summary>
    EntryContext: Option<ExecutionContext>,
    /// <summary>
    /// The stack to store the return values.
    /// </summary>
    ResultStack: EvaluationStack,

    /// <summary>
    /// The VM object representing the uncaught exception.
    /// </summary>
    UncaughtException: Option<StackItem>,// { get; private set; }
}

impl ExecutionEngine {
    pub fn set_state(&mut self, value: VMState) {
        if self.state != value
        {
            self.state = value;
            OnStateChanged();
        }
    }

    pub fn set_isJumping(&mut self, isJumping: bool) {
        self.isJumping = isJumping;
    }

    pub fn set_Limits(&mut self, Limits: _) {
        self.Limits = Limits;
    }

    pub fn set_ReferenceCounter(&mut self, ReferenceCounter: _) {
        self.ReferenceCounter = ReferenceCounter;
    }

    pub fn set_InvocationStack(&mut self, InvocationStack: _) {
        self.InvocationStack = InvocationStack;
    }

    pub fn set_CurrentContext(&mut self, CurrentContext: Option<_>) {
        self.CurrentContext = CurrentContext;
    }

    pub fn set_EntryContext(&mut self, EntryContext: Option<_>) {
        self.EntryContext = EntryContext;
    }

    pub fn set_ResultStack(&mut self, ResultStack: _) {
        self.ResultStack = ResultStack;
    }

    pub fn state(&self) -> &VMState {
        &self.state
    }
    pub fn isJumping(&self) -> bool {
        self.isJumping
    }
    pub fn Limits(&self) -> _ {
        self.Limits
    }
    pub fn ReferenceCounter(&self) -> _ {
        self.ReferenceCounter
    }
    pub fn InvocationStack(&self) -> _ {
        self.InvocationStack
    }
    pub fn CurrentContext(&self) -> &Option<_> {
        &self.CurrentContext
    }
    pub fn EntryContext(&self) -> &Option<_> {
        &self.EntryContext
    }
    pub fn ResultStack(&self) -> _ {
        self.ResultStack
    }
}

// namespace Neo.VM
// {
/// <summary>
/// Represents the VM used to execute the script.
/// </summary>
impl ExecutionEngine
{
    /// <summary>
    /// Initializes a new instance of the <see cref="ExecutionEngine"/> class.
    /// </summary>
    // fn ExecutionEngine(): this(new ReferenceCounter(), ExecutionEngineLimits.Default)
    // {}

    /// <summary>
    /// Initializes a new instance of the <see cref="ExecutionEngine"/> class with the specified <see cref="VM.ReferenceCounter"/> and <see cref="ExecutionEngineLimits"/>.
    /// </summary>
    /// <param name="referenceCounter">The reference counter to be used.</param>
    /// <param name="limits">Restrictions on the VM.</param>

    // fn ExecutionEngine(ReferenceCounter referenceCounter, ExecutionEngineLimits limits)
    // {
    // this.Limits = limits;
    // this.ReferenceCounter = referenceCounter;
    // this.ResultStack = new EvaluationStack(referenceCounter);
    // }

    /// <summary>
    /// Called when a context is unloaded.
    /// </summary>
    /// <param name="context">The context being unloaded.</param>
    pub fn ContextUnloaded(context: ExecutionContext)
    {
        if (InvocationStack.Count == 0)
        {
            CurrentContext = null;
            EntryContext = null;
        } else {
            CurrentContext = InvocationStack.Peek();
        }
        if context.StaticFields != null && context.StaticFields != CurrentContext?.StaticFields
        {
            context.StaticFields.ClearReferences();
        }
        context.LocalVariables?.ClearReferences();
        context.Arguments?.ClearReferences();
    }

    pub fn Dispose(&self)
    {
        self.InvocationStack.Clear();
    }

    /// <summary>
    /// Start execution of the VM.
    /// </summary>
    /// <returns></returns>
    pub fn Execute() -> VMState
    {
        if State == VMState.BREAK
        { State = VMState.NONE; }

        while State != VMState.HALT && State != VMState.FAULT {
            ExecuteNext();
        }
        return State;
    }

    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    fn ExecuteCall(&mut self, position: usize)
    {
        self::LoadContext(self.CurrentContext.unwrap().Clone(position));
    }

    fn ExecuteInstruction(&mut self)
    {
        let mut instruction = self.CurrentContext.unwrap().CurrentInstruction();

        match instruction.OpCode {
            OpCode::PUSHINT8 |
            OpCode::PUSHINT16 |
            OpCode::PUSHINT32 |
            OpCode::PUSHINT64 |
            OpCode::PUSHINT128 |
            OpCode::PUSHINT256 => {
                Push(BigInt(instruction.Operand.Span));
            }
            OpCode::PUSHA => {
                let mut position: usize = checked(CurrentContext.InstructionPointer + instruction.TokenI32());
                if position < 0 || position > self.CurrentContext.unwrap().Script.Length() { panic!(); }
                // throw
                // new
                // InvalidOperationException($ "Bad pointer address: {position}");
                Push(new Pointer(CurrentContext.Script, position));
            }
            OpCode::PUSHNULL =>
                {
                    Push(StackItem.Null);
                }
            OpCode::PUSHDATA1 |
            OpCode::PUSHDATA2 |
            OpCode::PUSHDATA4 => {
                Limits.AssertMaxItemSize(instruction.Operand.Length);
                Push(instruction.Operand);
            }
            OpCode::PUSHM1 |
            OpCode::PUSH0 |
            OpCode::PUSH1 |
            OpCode::PUSH2 |
            OpCode::PUSH3 |
            OpCode::PUSH4 |
            OpCode::PUSH5 |
            OpCode::PUSH6 |
            OpCode::PUSH7 |
            OpCode::PUSH8 |
            OpCode::PUSH9 |
            OpCode::PUSH10 |
            OpCode::PUSH11 |
            OpCode::PUSH12 |
            OpCode::PUSH13 |
            OpCode::PUSH14 |
            OpCode::PUSH15 |
            OpCode::PUSH16 => {
                Push((int)instruction.OpCode - (int)OpCode.PUSH0);
            }
            OpCode::NOP => break,
            OpCode::JMP =>
                {
                    ExecuteJumpOffset(instruction.TokenI8());
                }
            OpCode::JMP_L =>
                {
                    ExecuteJumpOffset(instruction.TokenI32());
                }
            OpCode::JMPIF =>
                {
                    if (Pop().GetBoolean())
                    { ExecuteJumpOffset(instruction.TokenI8()); }
                }
            OpCode::JMPIF_L =>
                {
                    if (Pop().GetBoolean())
                    { ExecuteJumpOffset(instruction.TokenI32()); }
                }
            OpCode::JMPIFNOT =>
                {
                    if (!Pop().GetBoolean()) {
                        ExecuteJumpOffset(instruction.TokenI8());
                    }
                }
            OpCode::JMPIFNOT_L =>
                {
                    if (!Pop().GetBoolean()) {
                        ExecuteJumpOffset(instruction.TokenI32());
                    }
                }
            OpCode::JMPEQ =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 == x2 {
                        ExecuteJumpOffset(instruction.TokenI8());
                    }
                }
            OpCode::JMPEQ_L =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 == x2 {
                        ExecuteJumpOffset(instruction.TokenI32());
                    }
                }
            OpCode::JMPNE =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 != x2 {
                        ExecuteJumpOffset(instruction.TokenI8());
                    }
                }
            OpCode::JMPNE_L =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 != x2 { ExecuteJumpOffset(instruction.TokenI32()); }
                }
            OpCode::JMPGT =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if (x1 > x2) { ExecuteJumpOffset(instruction.TokenI8()); }
                }
            OpCode::JMPGT_L =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 > x2 { ExecuteJumpOffset(instruction.TokenI32()); }
                }
            OpCode::JMPGE =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 >= x2 { ExecuteJumpOffset(instruction.TokenI8()); }
                }
            OpCode::JMPGE_L =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 >= x2 { ExecuteJumpOffset(instruction.TokenI32()); }
                }
            OpCode::JMPLT =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 < x2 { ExecuteJumpOffset(instruction.TokenI8()); }
                }
            OpCode::JMPLT_L =>
                {
                    let x2 = Pop().GetInteger();
                    let x1 = Pop().GetInteger();
                    if x1 < x2 { ExecuteJumpOffset(instruction.TokenI32()); }
                }
            OpCode::JMPLE =>
                {
                    let x2 = Pop().GetInteger() as i32;
                    let x1 = Pop().GetInteger() as i32;
                    if x1 <= x2 {
                        ExecuteJumpOffset(instruction.TokenI8());
                    }
                }
            OpCode::JMPLE_L =>
                {
                    let x2 = Pop().GetInteger() as i32;
                    let x1 = Pop().GetInteger() as i32;
                    if x1 <= x2 { ExecuteJumpOffset(instruction.TokenI32()); }
                }
            OpCode::CALL =>
                {
                    ExecuteCall(checked(CurrentContext.InstructionPointer + instruction.TokenI8()));
                }
            OpCode::CALL_L =>
                {
                    ExecuteCall(checked(CurrentContext.InstructionPointer + instruction.TokenI32()));
                }
            OpCode::CALLA =>
                {
                    let x = Pop < Pointer > ();
                    if (x.Script != CurrentContext.Script) { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException("Pointers can't be shared between scripts");
                    ExecuteCall(x.Position);
                }
            OpCode::CALLT =>
                {
                    LoadToken(instruction.TokenU16);
                }
            OpCode::ABORT =>
                {
                    panic!();
                    // throw
                    // new
                    // Exception($ "{OpCode.ABORT} is executed.");
                }
            OpCode::ASSERT =>
                {
                    let x = Pop().GetBoolean() as bool;
                    if !x { panic!(); }
                    // throw
                    // new
                    // Exception($ "{OpCode.ASSERT} is executed with false result.");
                }
            OpCode::THROW =>
                {
                    ExecuteThrow(Pop());
                }
            OpCode::TRY =>
                {
                    let catchOffset = instruction.TokenI8() as i32;
                    let finallyOffset = instruction.TokenI8_1 as i32;
                    ExecuteTry(catchOffset, finallyOffset);
                }
            OpCode::TRY_L =>
                {
                    let catchOffset = instruction.TokenI32() as i32;
                    let finallyOffset = instruction.TokenI32_1 as i32;
                    ExecuteTry(catchOffset, finallyOffset);
                }
            OpCode::ENDTRY =>
                {
                    let endOffset = instruction.TokenI8() as i32;
                    ExecuteEndTry(endOffset);
                }
            OpCode::ENDTRY_L =>
                {
                    let endOffset = instruction.TokenI32() as i32;
                    ExecuteEndTry(endOffset);
                }
            OpCode::ENDFINALLY =>
                {
                    if self.CurrentContext.unwrap().TryStack().is_none() { panic!(); }

                    // throw
                    // new
                    // InvalidOperationException($ "The corresponding TRY block cannot be found.");
                    if (!CurrentContext.TryStack.TryPop(out ExceptionHandlingContext? currentTry)) { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The corresponding TRY block cannot be found.");

                    if (self.UncaughtException.is_none()) {
                        CurrentContext.InstructionPointer = currentTry.EndPointer;
                    } else {
                        HandleException();
                    }

                    self.isJumping = true;
                }
            OpCode::RET =>
                {
                    ExecutionContext
                    context_pop = InvocationStack.Pop();
                    EvaluationStack
                    stack_eval = InvocationStack.Count == 0?
                    ResultStack => InvocationStack.Peek().EvaluationStack;
                    if (context_pop.EvaluationStack != stack_eval)
                    {
                        if (context_pop.RVCount > = 0 & &context_pop.EvaluationStack.Count != context_pop.RVCount)
                        throw
                        new
                        InvalidOperationException("RVCount doesn't match with EvaluationStack");
                        context_pop.EvaluationStack.CopyTo(stack_eval);
                    }
                    if (InvocationStack.Count == 0)
                    State = VMState.HALT;
                    ContextUnloaded(context_pop);
                    self.isJumping = true;
                }
            OpCode::SYSCALL =>
                {
                    OnSysCall(instruction.TokenU32);
                }

            // Stack ops
            OpCode::DEPTH =>
                {
                    Push(CurrentContext.EvaluationStack.Count);
                }
            OpCode::DROP =>
                {
                    Pop();
                }
            OpCode::NIP =>
                {
                    CurrentContext.EvaluationStack.Remove < StackItem > (1);
                }
            OpCode::XDROP =>
                {
                    int
                    n = (int)
                    Pop().GetInteger();
                    if (n < 0)
                    throw
                    new
                    InvalidOperationException($ "The negative value {n} is invalid for OpCode.{instruction.OpCode}.");
                    CurrentContext.EvaluationStack.Remove < StackItem > (n);
                }
            OpCode::CLEAR =>
                {
                    CurrentContext.EvaluationStack.Clear();
                }
            OpCode::DUP =>
                {
                    Push(Peek());
                }
            OpCode::OVER =>
                {
                    Push(Peek(1));
                }
            OpCode::PICK =>
                {
                    int
                    n = (int)
                    Pop().GetInteger();
                    if (n < 0)
                    throw
                    new
                    InvalidOperationException($ "The negative value {n} is invalid for OpCode.{instruction.OpCode}.");
                    Push(Peek(n));
                }
            OpCode::TUCK =>
                {
                    CurrentContext.EvaluationStack.Insert(2, Peek());
                }
            OpCode::SWAP =>
                {
                    let x = CurrentContext.EvaluationStack.Remove < StackItem > (1);
                    Push(x);
                }
            OpCode::ROT =>
                {
                    let x = CurrentContext.EvaluationStack.Remove < StackItem > (2);
                    Push(x);
                }
            OpCode::ROLL =>
                {
                    int
                    n = (int)
                    Pop().GetInteger();
                    if (n < 0)
                    throw
                    new
                    InvalidOperationException($ "The negative value {n} is invalid for OpCode.{instruction.OpCode}.");
                    if (n == 0)

                    let x = CurrentContext.EvaluationStack.Remove < StackItem > (n);
                    Push(x);
                }
            OpCode::REVERSE3 =>
                {
                    CurrentContext.EvaluationStack.Reverse(3);
                }
            OpCode::REVERSE4 =>
                {
                    CurrentContext.EvaluationStack.Reverse(4);
                }
            OpCode::REVERSEN =>
                {
                    int
                    n = (int)
                    Pop().GetInteger();
                    CurrentContext.EvaluationStack.Reverse(n);
                }

            //Slot
            OpCode::INITSSLOT =>
                {
                    if (CurrentContext.StaticFields != null)
                    throw
                    new
                    InvalidOperationException($ "{instruction.OpCode} cannot be executed twice.");
                    if (instruction.TokenU8 == 0)
                    throw
                    new
                    InvalidOperationException($ "The operand {instruction.TokenU8} is invalid for OpCode.{instruction.OpCode}.");
                    CurrentContext.StaticFields = new
                    Slot(instruction.TokenU8, ReferenceCounter);
                }
            OpCode::INITSLOT =>
                {
                    if (CurrentContext.LocalVariables != null | | CurrentContext.Arguments != null)
                    throw
                    new
                    InvalidOperationException($ "{instruction.OpCode} cannot be executed twice.");
                    if (instruction.TokenU16 == 0)
                    throw
                    new
                    InvalidOperationException($ "The operand {instruction.TokenU16} is invalid for OpCode.{instruction.OpCode}.");
                    if (instruction.TokenU8 > 0)
                    {
                        CurrentContext.LocalVariables = new
                        Slot(instruction.TokenU8, ReferenceCounter);
                    }
                    if (instruction.TokenU8_1 > 0)
                    {
                        StackItem
                        []
                        items = new
                        StackItem[instruction.TokenU8_1];
                        for (int i = 0; i < instruction.TokenU8_1; i + +)
                        {
                            items[i] = Pop();
                        }
                        CurrentContext.Arguments = new
                        Slot(items, ReferenceCounter);
                    }
                }
            OpCode::LDSFLD0 =>
                OpCode::LDSFLD1:
                    OpCode::LDSFLD2:
                    OpCode::LDSFLD3:
                    OpCode::LDSFLD4:
                    OpCode::LDSFLD5:
                    OpCode::LDSFLD6 =>
            {
            ExecuteLoadFromSlot(CurrentContext.StaticFields, instruction.OpCode - OpCode.LDSFLD0);

            }
            OpCode::LDSFLD =>
                {
                    ExecuteLoadFromSlot(CurrentContext.StaticFields, instruction.TokenU8);
                }
            OpCode::STSFLD0 |
            OpCode::STSFLD1 |
            OpCode::STSFLD2 |
            OpCode::STSFLD3 |
            OpCode::STSFLD4 |
            OpCode::STSFLD5 |
            OpCode::STSFLD6 =>
                {
                    ExecuteStoreToSlot(CurrentContext.StaticFields, instruction.OpCode - OpCode.STSFLD0);
                }
            OpCode::STSFLD =>
                {
                    ExecuteStoreToSlot(CurrentContext.StaticFields, instruction.TokenU8);
                }
            OpCode::LDLOC0 |
            OpCode::LDLOC1 |
            OpCode::LDLOC2 |
            OpCode::LDLOC3 |
            OpCode::LDLOC4 |
            OpCode::LDLOC5 |
            OpCode::LDLOC6 =>
                {
                    ExecuteLoadFromSlot(CurrentContext.LocalVariables, instruction.OpCode - OpCode.LDLOC0);
                }
            OpCode::LDLOC =>
                {
                    ExecuteLoadFromSlot(CurrentContext.LocalVariables, instruction.TokenU8);
                }
            OpCode::STLOC0 |
            OpCode::STLOC1 |
            OpCode::STLOC2 |
            OpCode::STLOC3 |
            OpCode::STLOC4 |
            OpCode::STLOC5 |
            OpCode::STLOC6 =>
                {
                    ExecuteStoreToSlot(CurrentContext.LocalVariables, instruction.OpCode - OpCode.STLOC0);
                }
            OpCode::STLOC =>
                {
                    ExecuteStoreToSlot(CurrentContext.LocalVariables, instruction.TokenU8);
                }
            OpCode::LDARG0 |
            OpCode::LDARG1 |
            OpCode::LDARG2 |
            OpCode::LDARG3 |
            OpCode::LDARG4 |
            OpCode::LDARG5 |
            OpCode::LDARG6 =>
                {
                    ExecuteLoadFromSlot(CurrentContext.Arguments, instruction.OpCode - OpCode.LDARG0);
                }
            OpCode::LDARG =>
                {
                    ExecuteLoadFromSlot(CurrentContext.Arguments, instruction.TokenU8);
                }
            OpCode::STARG0 |
            OpCode::STARG1 |
            OpCode::STARG2 |
            OpCode::STARG3 |
            OpCode::STARG4 |
            OpCode::STARG5 |
            OpCode::STARG6 =>
                {
                    ExecuteStoreToSlot(CurrentContext.Arguments, instruction.OpCode - OpCode.STARG0);
                }
            OpCode::STARG =>
                {
                    ExecuteStoreToSlot(CurrentContext.Arguments, instruction.TokenU8);
                }

            // Splice
            OpCode::NEWBUFFER =>
                {
                    int
                    length = (int)
                    Pop().GetInteger();
                    Limits.AssertMaxItemSize(length);
                    Push(new Buffer(length));
                }
            OpCode::MEMCPY =>
                {
                    let count = Pop().GetInteger() as i32;
                    if count < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    let si = Pop().GetInteger() as i32;
                    if si < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {si} is out of range.");
                    ReadOnlySpan < u8 > src = Pop().GetSpan();
                    if (checked(si + count) > src.Length) { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    let di = Pop().GetInteger() as i32;
                    if di < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {di} is out of range.");
                    Buffer
                    dst = Pop < Buffer > ();
                    if checked(di + count) > dst.Size { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    src.Slice(si, count).CopyTo(dst.InnerBuffer.AsSpan(di));
                }
            OpCode::CAT =>
                {
                    let x2 = Pop().GetSpan();
                    let x1 = Pop().GetSpan();
                    let length = x1.Length + x2.Length;
                    Limits.AssertMaxItemSize(length);
                    Buffer
                    result = new(length);
                    x1.CopyTo(result.InnerBuffer);
                    x2.CopyTo(result.InnerBuffer.AsSpan(x1.Length));
                    Push(result);
                }
            OpCode::SUBSTR =>
                {
                    let count = Pop().GetInteger() as i32;
                    if count < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    let index = Pop().GetInteger() as i32;
                    if index < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {index} is out of range.");
                    let x = Pop().GetSpan();
                    if index + count > x.Length { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    Buffer
                    result = new(count);
                    x.Slice(index, count).CopyTo(result.InnerBuffer);
                    Push(result);
                }
            OpCode::LEFT =>
                {
                    let count = Pop().GetInteger() as usize;
                    if count < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    let x = Pop().GetSpan();
                    if count > x.Length { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    Buffer
                    result = new(count);
                    x[..count].CopyTo(result.InnerBuffer);
                    Push(result);
                }
            OpCode::RIGHT =>
                {
                    let count = Pop().GetInteger() as i32;
                    if count < 0 { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    let x = Pop().GetSpan();
                    if count > x.Length { panic!(); }
                    // throw
                    // new
                    // InvalidOperationException($ "The value {count} is out of range.");
                    Buffer
                    result = new(count);
                    x
                    [ ^ count.. ^ 0].CopyTo(result.InnerBuffer);
                    Push(result);
                }
            // Bitwise logic
            OpCode::INVERT =>
                {
                    let x = Pop().GetInteger();
                    Push(~x);
                }
            OpCode::AND =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 & x2);
                }
            OpCode::OR =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 | x2);
                }
            OpCode::XOR =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 ^ x2);
                }
            OpCode::EQUAL =>
                {
                    StackItem
                    x2 = Pop();
                    StackItem
                    x1 = Pop();
                    Push(x1.Equals(x2));
                }
            OpCode::NOTEQUAL =>
                {
                    StackItem
                    x2 = Pop();
                    StackItem
                    x1 = Pop();
                    Push(!x1.Equals(x2));
                }

            // Numeric
            OpCode::SIGN =>
                {
                    let
                        x = Pop().GetInteger();
                    Push(x.Sign);
                }
            OpCode::ABS =>
                {
                    let
                        x = Pop().GetInteger();
                    Push(BigInt.Abs(x));
                }
            OpCode::NEGATE =>
                {
                    let
                        x = Pop().GetInteger();
                    Push(-x);
                }
            OpCode::INC =>
                {
                    let
                        x = Pop().GetInteger();
                    Push(x + 1);
                }
            OpCode::DEC =>
                {
                    let
                        x = Pop().GetInteger();
                    Push(x - 1);
                }
            OpCode::ADD =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 + x2);
                }
            OpCode::SUB =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 - x2);
                }
            OpCode::MUL =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 * x2);
                }
            OpCode::DIV =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 / x2);
                }
            OpCode::MOD =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 % x2);
                }
            OpCode::POW =>
                {
                    let
                        exponent = (int)
                    Pop().GetInteger();
                    let
                        value = Pop().GetInteger();
                    Push(BigInt.Pow(value, exponent));
                }
            OpCode::SQRT =>
                {
                    Push(Pop().GetInteger().Sqrt());
                }
            OpCode::SHL =>
                {
                    int
                    shift = (int)
                    Pop().GetInteger();
                    Limits.AssertShift(shift);
                    if (shift == 0)

                    let
                        x = Pop().GetInteger();
                    Push(x < < shift);
                }
            OpCode::SHR =>
                {
                    int
                    shift = (int)
                    Pop().GetInteger();
                    Limits.AssertShift(shift);
                    if (shift == 0)

                    let
                        x = Pop().GetInteger();
                    Push(x > > shift);
                }
            OpCode::NOT =>
                {
                    let
                        x = Pop().GetBoolean();
                    Push(!x);
                }
            OpCode::BOOLAND =>
                {
                    let
                        x2 = Pop().GetBoolean();
                    let
                        x1 = Pop().GetBoolean();
                    Push(x1 & &x2);
                }
            OpCode::BOOLOR =>
                {
                    let
                        x2 = Pop().GetBoolean();
                    let
                        x1 = Pop().GetBoolean();
                    Push(x1 | | x2);
                }
            OpCode::NZ =>
                {
                    let
                        x = Pop().GetInteger();
                    Push(!x.IsZero);
                }
            OpCode::NUMEQUAL =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 == x2);
                }
            OpCode::NUMNOTEQUAL =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(x1 != x2);
                }
            OpCode::LT =>
                {
                    let
                        x2 = Pop();
                    let
                        x1 = Pop();
                    if (x1.IsNull || x2.IsNull)
                    Push(false);
                    else
                    Push(x1.GetInteger() < x2.GetInteger());
                }
            OpCode::LE =>
                {
                    let
                        x2 = Pop();
                    let
                        x1 = Pop();
                    if (x1.IsNull || x2.IsNull)
                    Push(false);
                    else
                    Push(x1.GetInteger() < = x2.GetInteger());
                }
            OpCode::GT =>
                {
                    let
                        x2 = Pop();
                    let
                        x1 = Pop();
                    if (x1.IsNull || x2.IsNull)
                    Push(false);
                    else
                    Push(x1.GetInteger() > x2.GetInteger());
                }
            OpCode::GE =>
                {
                    let
                        x2 = Pop();
                    let
                        x1 = Pop();
                    if (x1.IsNull || x2.IsNull)
                    Push(false);
                    else
                    Push(x1.GetInteger() > = x2.GetInteger());
                }
            OpCode::MIN =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(BigInt.Min(x1, x2));
                }
            OpCode::MAX =>
                {
                    let
                        x2 = Pop().GetInteger();
                    let
                        x1 = Pop().GetInteger();
                    Push(BigInt.Max(x1, x2));
                }
            OpCode::WITHIN =>
                {
                    BigInt
                    b = Pop().GetInteger();
                    BigInt
                    a = Pop().GetInteger();
                    let
                        x = Pop().GetInteger();
                    Push(a < = x && x < b);
                }

            // Compound-type
            OpCode::PACK =>
                {
                    int
                    size = (int)
                    Pop().GetInteger();
                    if (size < 0 || size > CurrentContext.EvaluationStack.Count)
                    throw
                    new
                    InvalidOperationException($ "The value {size} is out of range.");
                    VMArray
                    array = new(ReferenceCounter);
                    for (int i = 0; i < size; i + +)
                    {
                        StackItem
                        item = Pop();
                        array.Add(item);
                    }
                    Push(array);
                }
            OpCode::UNPACK =>
                {
                    VMArray
                    array = Pop < VMArray > ();
                    for (int i = array.Count -1; i > = 0; i - -)
                    Push(array[i]);
                    Push(array.Count);
                }
            OpCode::NEWARRAY0 =>
                {
                    Push(new VMArray(ReferenceCounter));
                }
            OpCode::NEWARRAY |
            OpCode::NEWARRAY_T =>
                {
                    int
                    n = (int)
                    Pop().GetInteger();
                    if (n < 0 || n > Limits.MaxStackSize)
                    throw
                    new
                    InvalidOperationException($ "MaxStackSize exceed: {n}");
                    StackItem
                    item;
                    if (instruction.OpCode == OpCode.NEWARRAY_T)
                    {
                        StackItemType type = (StackItemType)
                        instruction.TokenU8;
                        if (!Enum.IsDefined(typeof(StackItemType), type ))
                        throw
                        new
                        InvalidOperationException($ "Invalid type for {instruction.OpCode}: {instruction.TokenU8}");
                        item = instruction.TokenU8
                        switch
                        {
                            (byte)StackItemType.Boolean => StackItem.False,
                            (byte)StackItemType.Integer => Integer.Zero,
                            (byte)StackItemType.ByteString => ByteString.Empty,
                            _ => StackItem.Null,
                        };
                    } else {
                        item = StackItem.Null;
                    }
                    Push(new VMArray(ReferenceCounter, Enumerable.Repeat(item, n)));
                }
            OpCode::NEWSTRUCT0 =>
                {
                    Push(new Struct(ReferenceCounter));
                }
            OpCode::NEWSTRUCT =>
                {
                    int
                    n = (int)
                    Pop().GetInteger();
                    if (n < 0 || n > Limits.MaxStackSize)
                    throw
                    new
                    InvalidOperationException($ "MaxStackSize exceed: {n}");
                    Struct
                    result = new(ReferenceCounter);
                    for (let i = 0; i < n; i + +)
                    result.Add(StackItem.Null);
                    Push(result);
                }
            OpCode::NEWMAP =>
                {
                    Push(new Map(ReferenceCounter));
                }
            OpCode::SIZE =>
                {
                    let
                        x = Pop();
                    switch(x)
                    {
                        case
                        CompoundType
                        compound =>
                        Push(compound.Count);
                        |
                        case
                        PrimitiveType
                        primitive =>
                        Push(primitive.Size);
                        |
                        case
                        Buffer
                        buffer =>
                        Push(buffer.Size);
                        |
                        default =>
                        throw
                        new
                        InvalidOperationException($ "Invalid type for {instruction.OpCode}=> {x.Type}");
                    }
                    |
                }
            OpCode::HASKEY =>
                {
                    PrimitiveType
                    key = Pop < PrimitiveType > ();
                    let
                        x = Pop();
                    switch(x)
                    {
                        case
                        VMArray
                        array =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0)
                            throw
                            new
                            InvalidOperationException($ "The negative value {index} is invalid for OpCode.{instruction.OpCode}.");
                            Push(index < array.Count);
                            |
                        }
                        case
                        Map
                        map =>
                        {
                            Push(map.ContainsKey(key));
                            |
                        }
                        case
                        Buffer
                        buffer =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0)
                            throw
                            new
                            InvalidOperationException($ "The negative value {index} is invalid for OpCode.{instruction.OpCode}.");
                            Push(index < buffer.Size);
                            |
                        }
                        case
                        ByteString
                        array =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0)
                            throw
                            new
                            InvalidOperationException($ "The negative value {index} is invalid for OpCode.{instruction.OpCode}.");
                            Push(index < array.Size);
                            |
                        }
                        default =>
                        throw
                        new
                        InvalidOperationException($ "Invalid type for {instruction.OpCode}=> {x.Type}");
                    }
                    |
                }
            OpCode::KEYS =>
                {
                    Map
                    map = Pop < Map > ();
                    Push(new VMArray(ReferenceCounter, map.Keys));
                    |
                }
            OpCode::VALUES =>
                {
                    let
                        x = Pop();
                    IEnumerable < StackItem > values = x
                    switch
                    {
                        VMArray array => array,
                        Map map => map.Values,
                        _ => throw new InvalidOperationException( $ "Invalid type for {instruction.OpCode}=> {x.Type}"),
                    };
                    VMArray
                    newArray = new(ReferenceCounter);
                    foreach(StackItem item in values)
                    if (item
                    is
                    Struct
                    s)
                    newArray.Add(s.Clone());
                    else
                    newArray.Add(item);
                    Push(newArray);
                    |
                }
            OpCode::PICKITEM =>
                {
                    PrimitiveType
                    key = Pop < PrimitiveType > ();
                    let
                        x = Pop();
                    switch(x)
                    {
                        case
                        VMArray
                        array =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0 | | index > = array.Count)
                            throw
                            new
                            InvalidOperationException($ "The value {index} is out of range.");
                            Push(array[index]);
                            |
                        }
                        case
                        Map
                        map =>
                        {
                            if (!map.TryGetValue(key, out StackItem? value))
                            throw
                            new
                            InvalidOperationException($ "Key not found in {nameof(Map)}");
                            Push(value);
                            |
                        }
                        case
                        PrimitiveType
                        primitive =>
                        {
                            ReadOnlySpan < byte > byteArray = primitive.GetSpan();
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0 || index > = byteArray.Length)
                            throw
                            new
                            InvalidOperationException($ "The value {index} is out of range.");
                            Push((BigInt)byteArray[index]);
                            |
                        }
                        case
                        Buffer
                        buffer =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0 | | index > = buffer.Size)
                            throw
                            new
                            InvalidOperationException($ "The value {index} is out of range.");
                            Push((BigInt)buffer.InnerBuffer[index]);
                            |
                        }
                        default =>
                        throw
                        new
                        InvalidOperationException($ "Invalid type for {instruction.OpCode}=> {x.Type}");
                    }
                    |
                }
            OpCode::APPEND =>
                {
                    StackItem
                    newItem = Pop();
                    VMArray
                    array = Pop < VMArray > ();
                    if (newItem
                    is
                    Struct
                    s) newItem = s.Clone();
                    array.Add(newItem);
                    |
                }
            OpCode::SETITEM =>
                {
                    StackItem
                    value = Pop();
                    if (value
                    is
                    Struct
                    s) value = s.Clone();
                    PrimitiveType
                    key = Pop < PrimitiveType > ();
                    let
                        x = Pop();
                    switch(x)
                    {
                        case
                        VMArray
                        array =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0 | | index > = array.Count)
                            throw
                            new
                            InvalidOperationException($ "The value {index} is out of range.");
                            array[index] = value;
                            |
                        }
                        case
                        Map
                        map =>
                        {
                            map[key] = value;
                            |
                        }
                        case
                        Buffer
                        buffer =>
                        {
                            int
                            index = (int)
                            key.GetInteger();
                            if (index < 0 | | index > = buffer.Size)
                            throw
                            new
                            InvalidOperationException($ "The value {index} is out of range.");
                            if (value
                            is
                            not
                            PrimitiveType
                            p)
                            throw
                            new
                            InvalidOperationException($ "Value must be a primitive type in {instruction.OpCode}");
                            int
                            b = (int)
                            p.GetInteger();
                            if (b < sbyte.MinValue || b > byte.MaxValue)
                            throw
                            new
                            InvalidOperationException($ "Overflow in {instruction.OpCode}, {b} is not a byte type.");
                            buffer.InnerBuffer[index] = (byte)
                            b;
                        }
                        default =>
                        // throw
                        // new
                        // InvalidOperationException($ "Invalid type for {instruction.OpCode}=> {x.Type}");
                    }
                }
            OpCode::REVERSEITEMS =>
                {
                    let
                        x = Pop();
                    switch(x)
                    {
                        case
                        VMArray
                        array =>
                        array.Reverse();
                        |
                        case
                        Buffer
                        buffer =>
                        Array.Reverse(buffer.InnerBuffer);
                        |
                        default =>
                        throw
                        new
                        InvalidOperationException($ "Invalid type for {instruction.OpCode}=> {x.Type}");
                    }
                    |
                }
            OpCode::REMOVE =>
                {
                    PrimitiveType
                    key = Pop < PrimitiveType > ();
                    let
                        x = Pop();
                    switch(x)
                    {
                        case
                        VMArray
                        array =>
                        int
                        index = (int)
                        key.GetInteger();
                        if (index < 0 | | index > = array.Count)
                        throw
                        new
                        InvalidOperationException($ "The value {index} is out of range.");
                        array.RemoveAt(index);
                        |
                        case
                        Map
                        map =>
                        map.Remove(key);
                        |
                        default =>
                        throw
                        new
                        InvalidOperationException($ "Invalid type for {instruction.OpCode}=> {x.Type}");
                    }
                }
            OpCode::CLEARITEMS =>
                {
                    CompoundType
                    x = Pop < CompoundType > ();
                    x.Clear();
                }
            OpCode::POPITEM =>
                {
                    VMArray
                    x = Pop < VMArray > ();
                    int
                    index = x.Count - 1;
                    Push(x[index]);
                    x.RemoveAt(index);
                    |
                }

            //Types
            OpCode::ISNULL =>
                {
                    let x = Pop();
                    Push(x.IsNull);
                    |
                }
            OpCode::ISTYPE =>
                {
                    let x = Pop();
                    StackItemType type = (StackItemType)
                    instruction.TokenU8;
                    if ( type == StackItemType.Any || !Enum.IsDefined(typeof(StackItemType), type ))
                    throw
                    new
                    InvalidOperationException($ "Invalid type: {type}");
                    Push(x.Type == type );
                    |
                }
            OpCode::CONVERT =>
                {
                    let x = Pop();
                    Push(x.ConvertTo((StackItemType)instruction.TokenU8));
                    |
                }

            _ => unreachable!();

        }
    }

    private void ExecuteEndTry(int endOffset)
    {
    if (CurrentContext !.TryStack is null)
    throw new InvalidOperationException( $ "The corresponding TRY block cannot be found.");
    if ( !CurrentContext.TryStack.TryPeek(out ExceptionHandlingContext ? currentTry))
    throw new InvalidOperationException( $ "The corresponding TRY block cannot be found.");
    if (currentTry.State == ExceptionHandlingState.Finally)
    throw new InvalidOperationException( $ "The opcode {OpCode.ENDTRY} can't be executed in a FINALLY block.");

    int endPointer = checked(CurrentContext.InstructionPointer + endOffset);
    if (currentTry.HasFinally)
    {
    currentTry.State = ExceptionHandlingState.Finally;
    currentTry.EndPointer = endPointer;
    CurrentContext.InstructionPointer = currentTry.FinallyPointer;
    }
    else
    {
    CurrentContext.TryStack.Pop();
    CurrentContext.InstructionPointer = endPointer;
    }
    self.isJumping = true;
    }

    /// <summary>
    /// Jump to the specified position.
    /// </summary>
    /// <param name="position">The position to jump to.</param>
    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    fn ExecuteJump(&self, position: i32)
    {
        if position < 0 || position > CurrentContext!.Script.Length { panic!(); }
        // throw new ArgumentOutOfRangeException($ "Jump out of range for position: {position}");
        CurrentContext.InstructionPointer = position;
        self.isJumping = true;
    }

    /// <summary>
    /// Jump to the specified offset from the current position.
    /// </summary>
    /// <param name="offset">The offset from the current position to jump to.</param>
    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    fn ExecuteJumpOffset(&self, offset: i32)
    {
        ExecuteJump(checked(CurrentContext!.InstructionPointer + offset));
    }

    fn ExecuteLoadFromSlot(slot: Option<Slot>, index: i32)
    {
        if (slot.is_none()) { panic!(); }
        // throw new InvalidOperationException("Slot has not been initialized.");
        if (index < 0 || index >= slot.unwrap().Count) { panic!(); }
        // throw new InvalidOperationException( $ "Index out of range when loading from slot: {index}");
        Push(slot[index]);
    }

    /// <summary>
    /// Execute the next instruction.
    /// </summary>
    fn ExecuteNext(&mut self)
    {
        if (InvocationStack.Count == 0)
        {
            State = VMState.HALT;
        } else {
            try
                {
                    ExecutionContext
                    context = CurrentContext!;
                    PreExecuteInstruction();
                    ExecuteInstruction();
                    PostExecuteInstruction();
                    if (!self.isJumping)
                    context.MoveNext();
                    self.isJumping = false;
                }
            catch(Exception e)
            {
                OnFault(e);
            }
        }
    }

    fn ExecuteStoreToSlot(&mut self, slot: Option<&Slot>, index: i32)
    {
        if (slot.is_none()) { panic!(); }
        // throw new InvalidOperationException("Slot has not been initialized.");
        if (index < 0 || index >= slot.Count) { panic!(); }
        // throw new InvalidOperationException( $ "Index out of range when storing to slot: {index}");
        slot[index] = Pop();
    }

    /// <summary>
    /// Throws a specified exception in the VM.
    /// </summary>
    /// <param name="ex">The exception to be thrown.</param>
    fn ExecuteThrow(&mut self, ex: &StackItem)
    {
        self.UncaughtException = ex;
        HandleException();
    }

    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    fn ExecuteTry(&mut self, catchOffset: i32, finallyOffset: i32)
    {
        if catchOffset == 0 && finallyOffset == 0 { panic!(); }
        // throw
        // new
        // InvalidOperationException($ "catchOffset and finallyOffset can't be 0 in a TRY block");
        if self.CurrentContext.unwrap().TryStack().is_none() {
            CurrentContext.TryStack = new
            Stack < ExceptionHandlingContext > ();
        } else if (CurrentContext.TryStack.Count > = Limits.MaxTryNestingDepth) { panic!(); }
        // throw
        // new
        // InvalidOperationException("MaxTryNestingDepth exceed.");
        int
        catchPointer = catchOffset == 0? - 1: checked(CurrentContext.InstructionPointer + catchOffset);
        int
        finallyPointer = finallyOffset == 0? - 1: checked(CurrentContext.InstructionPointer + finallyOffset);
        CurrentContext.TryStack.Push(new ExceptionHandlingContext(catchPointer, finallyPointer));
    }

    fn HandleException(&mut self)
    {
        let mut pop = 0;
        for (executionContext in self.InvocationStack)
        {
            if (executionContext.TryStack != null)
            {
                while (executionContext.TryStack.TryPeek(out var tryContext))
                {
                    if (tryContext.State == ExceptionHandlingState.Finally || (tryContext.State == ExceptionHandlingState.Catch && !tryContext.HasFinally))
                    {
                        executionContext.TryStack.Pop();
                        continue;
                    }
                    for i in 0..pop
                    {
                        ContextUnloaded(InvocationStack.Pop());
                    }
                    if tryContext.State == ExceptionHandlingState.Try && tryContext.HasCatch
                    {
                        tryContext.State = ExceptionHandlingState.Catch;
                        Push(UncaughtException!);
                        executionContext.InstructionPointer = tryContext.CatchPointer;
                        UncaughtException = null;
                    } else {
                        tryContext.State = ExceptionHandlingState.Finally;
                        executionContext.InstructionPointer = tryContext.FinallyPointer;
                    }
                    self.isJumping = true;
                    return;
                }
            }
            pop += 1;
        }

        // throw
        // new
        // VMUnhandledException(UncaughtException!);
    }

    /// <summary>
    /// Loads the specified context into the invocation stack.
    /// </summary>
    /// <param name="context">The context to load.</param>
    fn LoadContext(&mut self, context: &ExecutionContext)
    {
        if self.InvocationStack.Count >= self.Limits.MaxInvocationStackSize {panic!();}
        // throw
        // new
        // InvalidOperationException($ "MaxInvocationStackSize exceed: {InvocationStack.Count}");
        InvocationStack.Push(context);
        if (self.EntryContext.is_none()) {EntryContext = context;}

        self.CurrentContext = Some(context.clone());
    }

    /// <summary>
    /// Create a new context with the specified script without loading.
    /// </summary>
    /// <param name="script">The script used to create the context.</param>
    /// <param name="rvcount">The number of values that the context should return when it is unloaded.</param>
    /// <param name="initialPosition">The pointer indicating the current instruction.</param>
    /// <returns>The created context.</returns>
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    protected ExecutionContext CreateContext(Script script, int rvcount, int initialPosition)
    {
    return new ExecutionContext(script, rvcount, ReferenceCounter)
    {
    InstructionPointer = initialPosition
    };
    }

    /// <summary>
    /// Create a new context with the specified script and load it.
    /// </summary>
    /// <param name="script">The script used to create the context.</param>
    /// <param name="rvcount">The number of values that the context should return when it is unloaded.</param>
    /// <param name="initialPosition">The pointer indicating the current instruction.</param>
    /// <returns>The created context.</returns>
    pub fn LoadScript(script: &Script, rvcount: i32 = - 1,  initialPosition: i32 = 0) -> ExecutionContext
    {
        let mut context = CreateContext { script, rvcount, initialPosition };
        LoadContext(context);
        context
    }

    /// <summary>
    /// When overridden in a derived class, loads the specified method token.
    /// Called when <see cref="OpCode.CALLT"/> is executed.
    /// </summary>
    /// <param name="token">The method token to be loaded.</param>
    /// <returns>The created context.</returns>
    fn LoadToken(token: u16) -> ExecutionContext
    {
        panic!();
        // throw new InvalidOperationException( $ "Token not found: {token}");
    }

    /// <summary>
    /// Called when an exception that cannot be caught by the VM is thrown.
    /// </summary>
    /// <param name="ex">The exception that caused the <see cref="VMState.FAULT"/> state.</param>
    fn OnFault(ex: Exception)
    {
        State = VMState.FAULT;
    }

    /// <summary>
    /// Called when the state of the VM changed.
    /// </summary>
    fn OnStateChanged()
    {}

    /// <summary>
    /// When overridden in a derived class, invokes the specified system call.
    /// Called when <see cref="OpCode.SYSCALL"/> is executed.
    /// </summary>
    /// <param name="method">The system call to be invoked.</param>
    fn OnSysCall(method: u32)
    {
        panic!();
        // throw new InvalidOperationException( $ "Syscall not found: {method}");
    }

    /// <summary>
    /// Returns the item at the specified index from the top of the current stack without removing it.
    /// </summary>
    /// <param name="index">The index of the object from the top of the stack.</param>
    /// <returns>The item at the specified index.</returns>
    pub fn Peek(&self, index: Option<i32>) -> StackItem
    {
        match index {
            Some(val) => self.CurrentContext.unwrap().EvaluationStack.Peek(index),
            None => { self.CurrentContext.unwrap().EvaluationStack.Peek(0) }
        }
    }

    /// <summary>
    /// Removes and returns the item at the top of the current stack.
    /// </summary>
    /// <returns>The item removed from the top of the stack.</returns>
    pub fn Pop(&mut self) -> StackItem
    {
        self.CurrentContext.unwrap().EvaluationStack.Pop()
    }

    /// <summary>
    /// Removes and returns the item at the top of the current stack and convert it to the specified type.
    /// </summary>
    /// <typeparam name="T">The type to convert to.</typeparam>
    /// <returns>The item removed from the top of the stack.</returns>
    pub fn T Pop<T>() where T : StackItem
    {
    return CurrentContext !.EvaluationStack.Pop < T >();
    }

    /// <summary>
    /// Called after an instruction is executed.
    /// </summary>
    protected virtual void PostExecuteInstruction()
    {
    if (ReferenceCounter.CheckZeroReferred() > Limits.MaxStackSize)
    throw new InvalidOperationException( $ "MaxStackSize exceed: {ReferenceCounter.Count}");
    }

    /// <summary>
    /// Called before an instruction is executed.
    /// </summary>
    protected virtual void PreExecuteInstruction() {}

    /// <summary>
    /// Pushes an item onto the top of the current stack.
    /// </summary>
    /// <param name="item">The item to be pushed.</param>
    pub fn void Push(StackItem item)
    {
    CurrentContext !.EvaluationStack.Push(item);
    }
}
// }
