use std::any::Any;
use std::collections::HashMap;

use crate::ExceptionHandlingContext::ExceptionHandlingContext;
use crate::Instruction::Instruction;
use crate::Script::Script;
use crate::ReferenceCounter::ReferenceCounter;

struct SharedStates {
    Script: Script,
    EvaluationStack: EvaluationStack,
    StaticFields: Option<Slot>,
    States: HashMap<Type, dyn Any>,
}

// impl SharedStates
// {
//     public SharedStates(Script script, ReferenceCounter referenceCounter)
//     {
//     this.Script = script;
//     this.EvaluationStack = new EvaluationStack(referenceCounter);
//     this.States = new Dictionary < Type, object > ();
//     }
// }

pub struct ExecutionContext {
    shared_states: SharedStates,

    instructionPointer: i32,

    /// <summary>
    /// Indicates the number of values that the context should return when it is unloaded.
    /// </summary>
    RVCount: i32,// { get; }

    /// <summary>
    /// The script to run in this context.
    /// </summary>
    Script: Script,// => shared_states.Script;

    /// <summary>
    /// The evaluation stack for this context.
    /// </summary>
    pub(crate) EvaluationStack: EvaluationStack,// => shared_states.EvaluationStack;

    /// <summary>
    /// The slot used to store the static fields.
    /// </summary>
    pub(crate) StaticFields: Option<Slot>,

    /// <summary>
    /// The slot used to store the local variables of the current method.
    /// </summary>
    pub(crate) LocalVariables: Option<Slot>,

    /// <summary>
    /// The slot used to store the arguments of the current method.
    /// </summary>
    pub(crate) Arguments: Option<Slot>,

    /// <summary>
    /// The stack containing nested <see cref="ExceptionHandlingContext"/>.
    /// </summary>
    TryStack: Option<Stack<ExceptionHandlingContext>>,
    // { get; internal set; }
    pub initialPosition: i32,
}

impl ExecutionContext {
    pub fn shared_states(&self) -> &SharedStates {
        &self.shared_states
    }
    pub fn instructionPointer(&self) -> i32 {
        self.instructionPointer
    }
    pub fn RVCount(&self) -> i32 {
        self.RVCount
    }
    pub fn Script(&self) -> Script {
        self.Script
    }
    pub fn EvaluationStack(&self) -> _ {
        self.EvaluationStack
    }
    pub fn StaticFields(&self) -> &Option<_> {
        &self.StaticFields
    }
    pub fn LocalVariables(&self) -> &Option<_> {
        &self.LocalVariables
    }
    pub fn Arguments(&self) -> &Option<_> {
        &self.Arguments
    }
    pub fn TryStack(&self) -> &Option<_> {
        &self.TryStack
    }
}

impl ExecutionContext {
    pub fn set_shared_states(&mut self, shared_states: SharedStates) {
        self.shared_states = shared_states;
    }

    pub fn set_instructionPointer(&mut self, instructionPointer: i32) {
        if value < 0 || value > self.Script.Length() { panic!() }
        // throw new ArgumentOutOfRangeException(nameof(value));
        self.instructionPointer = value;
    }

    pub fn set_RVCount(&mut self, RVCount: i32) {
        self.RVCount = RVCount;
    }

    pub fn set_Script(&mut self, Script: Script) {
        self.Script = Script;
    }

    pub fn set_EvaluationStack(&mut self, EvaluationStack: _) {
        self.EvaluationStack = EvaluationStack;
    }

    pub fn set_StaticFields(&mut self, StaticFields: Option<_>) {
        self.StaticFields = StaticFields;
    }

    pub fn set_LocalVariables(&mut self, LocalVariables: Option<_>) {
        self.LocalVariables = LocalVariables;
    }

    pub fn set_Arguments(&mut self, Arguments: Option<_>) {
        self.Arguments = Arguments;
    }

    pub fn set_TryStack(&mut self, TryStack: Option<_>) {
        self.TryStack = TryStack;
    }
}


/// <summary>
/// Represents a frame in the VM execution stack.
/// </summary>
// [DebuggerDisplay("InstructionPointer={InstructionPointer}")]
impl ExecutionContext
{
    /// <summary>
    /// Returns the current <see cref="Instruction"/>.
    /// </summary>
    pub fn CurrentInstruction(&self) -> Instruction
    {
        GetInstruction(self.instructionPointer)
    }

    /// <summary>
    /// Returns the next <see cref="Instruction"/>.
    /// </summary>
    pub fn NextInstruction(&self) -> Instruction
    {
        GetInstruction(self.instructionPointer + &self.CurrentInstruction().Size)
    }

    fn ExecutionContext(script: &Script, rvcount: i32, referenceCounter: &ReferenceCounter)
    : this(new SharedStates(script, referenceCounter), rvcount, 0)
    {}

    fn ExecutionContext(SharedStates shared_states, int rvcount, int initialPosition)
    {
        if (rvcount < -1 | | rvcount > ushort.MaxValue)
        throw
        new
        ArgumentOutOfRangeException(nameof(rvcount));
        this.shared_states = shared_states;
        this.RVCount = rvcount;
        this.InstructionPointer = initialPosition;
    }

    /// <summary>
    /// Clones the context so that they share the same script, stack, and static fields.
    /// </summary>
    /// <returns>The cloned context.</returns>
    // pub fn Clone(&self) -> Self
    // {
    //     Clone(InstructionPointer);
    // }

    /// <summary>
    /// Clones the context so that they share the same script, stack, and static fields.
    /// </summary>
    /// <param name="initialPosition">The instruction pointer of the new context.</param>
    /// <returns>The cloned context.</returns>
    pub fn Clone(initialPosition: i32) -> ExecutionContext
    {
        ExecutionContext { shared_states, instructionPointer: 0, RVCount: 0, Script: (), EvaluationStack: (), StaticFields: None, LocalVariables: None, Arguments: None, 0, initialPosition, TryStack: None }
    }

    fn GetInstruction(ip: int) -> Instruction { Script.GetInstruction(ip); }

    /// <summary>
    /// Gets custom data of the specified type. If the data does not exist, create a new one.
    /// </summary>
    /// <typeparam name="T">The type of data to be obtained.</typeparam>
    /// <returns>The custom data of the specified type.</returns>
    pub fn GetState<T>() -> T
// where T : class, new()
    {
        if (!shared_states.States.TryGetValue(typeof(T), out object? value))
        {
            value = new
            T();
            shared_states.States
            [typeof(T)] = value;
        }
        return (T);
        value;
    }

    fn MoveNext(&self) -> bool
    {
        self.instructionPointer += CurrentInstruction.Size;
        return InstructionPointer < Script.Length;
    }
}

