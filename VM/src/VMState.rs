/// <summary>
/// Indicates the status of the VM.
/// </summary>
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum VMState
{
    /// <summary>
    /// Indicates that the execution is in progress or has not yet begun.
    /// </summary>
    NONE = 0,
    /// <summary>
    /// Indicates that the execution has been completed successfully.
    /// </summary>
    HALT = 1 << 0,
    /// <summary>
    /// Indicates that the execution has ended, and an exception that cannot be caught is thrown.
    /// </summary>
    FAULT = 1 << 1,
    /// <summary>
    /// Indicates that a breakpoint is currently being hit.
    /// </summary>
    BREAK = 1 << 2,
}

