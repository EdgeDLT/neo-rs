/// <summary>
/// Indicates the state of the <see cref="ExceptionHandlingContext"/>.
/// </summary>
pub enum ExceptionHandlingState
{
    /// <summary>
    /// Indicates that the <see langword="try"/> block is being executed.
    /// </summary>
    Try,

    /// <summary>
    /// Indicates that the <see langword="catch"/> block is being executed.
    /// </summary>
    Catch,

    /// <summary>
    /// Indicates that the <see langword="finally"/> block is being executed.
    /// </summary>
    Finally,
}