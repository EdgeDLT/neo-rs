use self::Types::*;

mod OpCode;
mod VMState;
mod Debugger;

mod Types;
mod Instruction;
mod Script;
mod ExecutionEngineLimits;
mod ExecutionEngine;
mod ExecutionContext;
mod ExceptionHandlingState;
mod ExceptionHandlingContext;
mod EvaluationStack;
mod BadScriptException;
mod OperandSizeAttribute;
mod ReferenceCounter;
mod Slot;
mod ScriptBuilder;
mod Unsafe;
mod VMUnhandledException;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
