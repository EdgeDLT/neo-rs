use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Getters, Setters, MutGetters, CopyGetters,Debug, Clone, Default, Eq, PartialEq)]
pub struct ExecutionEngineLimits {
    /// <summary>
    /// The maximum number of bits that <see cref="OpCode.SHL"/> and <see cref="OpCode.SHR"/> can shift.
    /// </summary>
    #[getset(get="pub", set)]
    max_shift: i32,

    /// <summary>
    /// The maximum number of items that can be contained in the VM's evaluation stacks and slots.
    /// </summary>
    #[getset(get="pub", set)]
    max_stack_size: u32,

    /// <summary>
    /// The maximum size of an item in the VM.
    /// </summary>
    #[getset(get="pub", set)]
    max_item_size: u32,

    /// <summary>
    /// The maximum number of frames in the invocation stack of the VM.
    /// </summary>
    #[getset(get="pub", set)]
    max_invocation_stack_size: u32,

    /// <summary>
    /// The maximum nesting depth of <see langword="try"/>-<see langword="catch"/>-<see langword="finally"/> blocks.
    /// </summary>
    #[getset(get="pub", set)]
    max_try_nesting_depth: u32,
}

impl Default for ExecutionEngineLimits {
    fn default() -> Self {
        Self {
            max_shift: 256,
            max_stack_size: 2 * 1024,
            max_item_size: 1024 * 1024,
            max_invocation_stack_size: 1024,
            max_try_nesting_depth: 16,
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
    fn assert_max_item_size(&self, size: u32)
    {
        if size < 0 || size > self.max_item_size
        {
            panic!();
            // throw new InvalidOperationException( $ "MaxItemSize exceed: {size}");
        }
    }

    /// <summary>
    /// Assert that the number of bits shifted meets the limit.
    /// </summary>
    /// <param name="shift">The number of bits shifted.</param>
    fn assert_shift(&self, shift: i32)
    {
        if shift > self.max_shift || shift < 0
        {
            panic!();
            // throw new InvalidOperationException( $ "Invalid shift value: {shift}");
        }
    }
}
