#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ExecutionEngineLimits {
    /// <summary>
/// The maximum number of bits that <see cref="OpCode.SHL"/> and <see cref="OpCode.SHR"/> can shift.
/// </summary>
    MaxShift: i32,

    /// <summary>
    /// The maximum number of items that can be contained in the VM's evaluation stacks and slots.
    /// </summary>
    MaxStackSize: u32,

    /// <summary>
    /// The maximum size of an item in the VM.
    /// </summary>
    MaxItemSize: u32,

    /// <summary>
    /// The maximum number of frames in the invocation stack of the VM.
    /// </summary>
    MaxInvocationStackSize: u32,

    /// <summary>
    /// The maximum nesting depth of <see langword="try"/>-<see langword="catch"/>-<see langword="finally"/> blocks.
    /// </summary>
    MaxTryNestingDepth: u32,
}

impl Default for ExecutionEngineLimits {
    fn default() -> Self {
        Self {
            MaxShift: 256,
            MaxStackSize: 2 * 1024,
            MaxItemSize: 1024 * 1024,
            MaxInvocationStackSize: 1024,
            MaxTryNestingDepth: 16,
        }
    }
}

/// <summary>
/// Represents the restrictions on the VM.
/// </summary>
impl ExecutionEngineLimits
{
    /// <summary>
    /// Assert that the size of the item meets the limit.
    /// </summary>
    /// <param name="size">The size to be checked.</param>
    fn AssertMaxItemSize(&self, size: u32)
    {
        if size < 0 || size > self.MaxItemSize
        {
            panic!();
            // throw new InvalidOperationException( $ "MaxItemSize exceed: {size}");
        }
    }

    /// <summary>
    /// Assert that the number of bits shifted meets the limit.
    /// </summary>
    /// <param name="shift">The number of bits shifted.</param>
    fn AssertShift(&self, shift: i32)
    {
        if shift > self.MaxShift || shift < 0
        {
            panic!();
            // throw new InvalidOperationException( $ "Invalid shift value: {shift}");
        }
    }
}
