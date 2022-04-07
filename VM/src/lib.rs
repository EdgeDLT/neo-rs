extern crate core;

use self::Types::*;
extern crate alloc;

pub mod OpCode;
pub mod VMState;
// pub mod Debugger;

pub mod Types;
pub mod Instruction;
pub mod Script;
pub mod ExecutionEngineLimits;
pub mod ExecutionEngine;
pub mod ExecutionContext;
pub mod ExceptionHandlingState;
pub mod ExceptionHandlingContext;
pub mod EvaluationStack;
pub mod BadScriptException;
pub mod ReferenceCounter;
pub mod Slot;
pub mod ScriptBuilder;
pub mod Unsafe;
mod VMUnhandledException;
mod Macros;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
