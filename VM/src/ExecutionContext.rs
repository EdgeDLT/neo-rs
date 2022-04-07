use std::any::Any;
use std::collections::HashMap;

use crate::EvaluationStack::EvaluationStack;
use crate::ExceptionHandlingContext::ExceptionHandlingContext;
use crate::Instruction::Instruction;
use crate::ReferenceCounter::ReferenceCounter;
use crate::Script::Script;
use crate::Slot::Slot;
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct SharedStates {
    script: Script,
    evaluation_stack: EvaluationStack,
    static_fields: Option<Slot>,
    states: HashMap<Type, dyn Any>,
}

impl SharedStates
{
    pub fn new( script:&Script,  referenceCounter:&ReferenceCounter)->Self
    {
        Self{
            script: script.clone(),
            evaluation_stack,
            static_fields: None,
            states: Dictionary<Type,object>()
        }
    }
}

#[derive(Getters, Setters, MutGetters, CopyGetters, Default)]
pub struct ExecutionContext {
    shared_states: SharedStates,

    #[getset(get = "pub", set)]
    instruction_pointer: i32,

    /// <summary>
    /// Indicates the number of values that the context should return when it is unloaded.
    /// </summary>
    #[getset(get_copy = "pub", set)]
    rv_count:i32,

    /// <summary>
    /// The slot used to store the local variables of the current method.
    /// </summary>
    #[getset(get_mut = "pub", set)]
    local_variables: Option<Slot>,

    /// <summary>
    /// The slot used to store the arguments of the current method.
    /// </summary>
    #[getset(get_copy = "pub", set, get_mut = "pub")]
    arguments: Option<Slot>,

    /// <summary>
    /// The stack containing nested <see cref="ExceptionHandlingContext"/>.
    /// </summary>
    // #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    #[getset(get = "pub", set)]
    try_stack: Option<Stack<ExceptionHandlingContext>>,
}

impl ExecutionContext {

    pub fn new(script: &Script, rvcount: i32, referenceCounter: &ReferenceCounter) ->Self{
        Self{
            shared_states: SharedStates::new(script, referenceCounter),
            instruction_pointer: 0,
            rv_count: rvcount,
            local_variables: None,
            arguments: None,
            try_stack: None,
        }
    }
    fn from_shared_states( shared_states:&SharedStates, rvcount:i32, initialPosition:i32) ->Self{
        Self{
            shared_states: shared_states.clone(),
            instruction_pointer: initialPosition,
            rv_count: rvcount,
            local_variables: None,
            arguments: None,
            try_stack: None,
        }
    }


    /// <summary>
    /// The script to run in this context.
    /// </summary>
    pub fn script(&self) ->&Script{
        &self.shared_states.script
    }

    pub fn script_mut(&mut self) ->&mut Script{
        &mut self.shared_states.script
    }

    /// <summary>
    /// The evaluation stack for this context.
    /// </summary>
    pub fn evaluation_stack(&mut self)->&EvaluationStack{
        &mut self.shared_states.EvaluationStack
    }

    /// <summary>
    /// The slot used to store the static fields.
    /// </summary>
    pub fn static_fields(&self) -> &Option<Slot> {
        &self.shared_states.static_fields
    }

    pub fn static_fields_mut(&mut self)->&mut Option<Slot>{
        &mut self.shared_states.static_fields
    }

    pub fn set_static_fields(&mut self, slots:Option<Slot>) {
        self.shared_states.static_fields = slots;
    }

    // pub fn set_instructionPointer(&mut self, instructionPointer: i32) -> &mut ExecutionContext {
    //     if value < 0 || value > self.Script.Length() { panic!() }
    //     // throw new ArgumentOutOfRangeException(nameof(value));
    //     self.instructionPointer = value;
    //     self
    // }

    /// <summary>
    /// Returns the current <see cref="Instruction"/>.
    /// </summary>
    pub fn current_instruction(&self) -> Instruction
    {
        self.instruction(self.instructionPointer)
    }

    /// <summary>
    /// Returns the next <see cref="Instruction"/>.
    /// </summary>
    pub fn next_instruction(&self) -> Instruction
    {
        self.instruction(self.instructionPointer + &self.CurrentInstruction().Size)
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
    pub fn clone(&self, initialPosition: i32) -> Self
    {
        Self { shared_states,
            instruction_pointer: 0,
            rv_count: 0,
            local_variables: None,
            arguments: None,
            try_stack: None }
    }

    fn instruction(&self, ip: i32) -> Instruction { &self.script().instruction(ip) }

    /// <summary>
    /// Gets custom data of the specified type. If the data does not exist, create a new one.
    /// </summary>
    /// <typeparam name="T">The type of data to be obtained.</typeparam>
    /// <returns>The custom data of the specified type.</returns>
    pub fn state<T>() -> T
// where T : class, new()
    {
        if !shared_states.States.TryGetValue(typeof(T), out object? value)
        {
            value = new T();
            shared_states.States[typeof(T)] = value;
        }
        return (T);
        value;
    }

    fn move_next(&mut self) -> bool
    {
        self.instruction_pointer += self.current_instruction.Size;
         self.instruction_pointer < self.script().Length
    }
}

