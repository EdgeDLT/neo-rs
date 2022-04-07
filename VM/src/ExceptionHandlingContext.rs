use crate::ExceptionHandlingState::ExceptionHandlingState;
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Getters, Setters, MutGetters, CopyGetters,Debug, Clone, Default, Eq, PartialEq)]
pub struct ExceptionHandlingContext {
    /// <summary>
    /// The position of the <see langword="catch"/> block.
    /// </summary>
    #[getset(get = "pub", set)]
    CatchPointer: i32,// { get; }

    /// <summary>
    /// The position of the <see langword="finally"/> block.
    /// </summary>
    #[getset(get = "pub", set, get_mut = "pub")]
    FinallyPointer: i32,//  { get; }

    /// <summary>
    /// The end position of the <see langword="try"/>-<see langword="catch"/>-<see langword="finally"/> block.
    /// </summary>
    #[getset(get = "pub", set, get_mut = "pub")]
    EndPointer: i32,//  { get; internal set; } = -1;

    /// <summary>
    /// Indicates whether the <see langword="catch"/> block is included in the context.
    /// </summary>
    #[getset(get = "pub", set, get_mut = "pub")]
    HasCatch: bool,// => CatchPointer >= 0;

    /// <summary>
    /// Indicates whether the <see langword="finally"/> block is included in the context.
    /// </summary>
    #[getset(get = "pub", set, get_mut = "pub")]
    HasFinally: bool,// => FinallyPointer >= 0;

    /// <summary>
    /// Indicates the state of the context.
    /// </summary>
    #[getset(get = "pub", set, get_mut = "pub")]
    State: ExceptionHandlingState,// { get; internal set; } = ExceptionHandlingState.Try;
}


/// <summary>
/// Represents the context used for exception handling.
/// </summary>
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
