use crate::ExceptionHandlingState::ExceptionHandlingState;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ExceptionHandlingContext {
    /// <summary>
    /// The position of the <see langword="catch"/> block.
    /// </summary>
    CatchPointer: i32,// { get; }

    /// <summary>
    /// The position of the <see langword="finally"/> block.
    /// </summary>
    FinallyPointer: i32,//  { get; }

    /// <summary>
    /// The end position of the <see langword="try"/>-<see langword="catch"/>-<see langword="finally"/> block.
    /// </summary>
    EndPointer: i32,//  { get; internal set; } = -1;

    /// <summary>
    /// Indicates whether the <see langword="catch"/> block is included in the context.
    /// </summary>
    HasCatch: bool,// => CatchPointer >= 0;

    /// <summary>
    /// Indicates whether the <see langword="finally"/> block is included in the context.
    /// </summary>
    HasFinally: bool,// => FinallyPointer >= 0;

    /// <summary>
    /// Indicates the state of the context.
    /// </summary>
    State: ExceptionHandlingState,// { get; internal set; } = ExceptionHandlingState.Try;
}

impl ExceptionHandlingContext {
    pub fn set_CatchPointer(&mut self, CatchPointer: i32) {
        self.CatchPointer = CatchPointer;
    }
    pub fn set_FinallyPointer(&mut self, FinallyPointer: i32) {
        self.FinallyPointer = FinallyPointer;
    }
    pub fn set_EndPointer(&mut self, EndPointer: i32) {
        self.EndPointer = EndPointer;
    }
    pub fn set_HasCatch(&mut self, HasCatch: bool) {
        self.HasCatch = HasCatch;
    }
    pub fn set_HasFinally(&mut self, HasFinally: bool) {
        self.HasFinally = HasFinally;
    }
    pub fn set_State(&mut self, State: ExceptionHandlingState) {
        self.State = State;
    }
}

impl ExceptionHandlingContext {
    pub fn CatchPointer(&self) -> i32 {
        self.CatchPointer
    }
    pub fn FinallyPointer(&self) -> i32 {
        self.FinallyPointer
    }
    pub fn EndPointer(&self) -> i32 {
        self.EndPointer
    }
    pub fn HasCatch(&self) -> bool {
        self.HasCatch
    }
    pub fn HasFinally(&self) -> bool {
        self.HasFinally
    }
    pub fn State(&self) -> ExceptionHandlingState {
        self.State.clone()
    }
}


/// <summary>
/// Represents the context used for exception handling.
/// </summary>
// [DebuggerDisplay("State={State}, CatchPointer={CatchPointer}, FinallyPointer={FinallyPointer}, EndPointer={EndPointer}")]
impl Default for ExceptionHandlingContext
{
    fn default() -> Self {
        Self {
            CatchPointer: 0,
            FinallyPointer: 0,
            EndPointer: -1,
            HasCatch: CatchPointer >= 0,
            HasFinally: FinallyPointer >= 0,
            State: ExceptionHandlingState.Try,
        }
    }
}
