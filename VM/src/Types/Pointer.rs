use crate::Script::Script;
use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;

/// <summary>
    /// Represents the instruction pointer in the VM, used as the target of jump instructions.
    /// </summary>
// [DebuggerDisplay("Type={GetType().Name}, Position={Position}")]
#[derive(Clone, Copy, Debug)]
pub struct Pointer {
    script: Script,
    position: usize,
}

impl Pointer {
    pub fn Script(&self) -> Script {
        self.script
    }
    pub fn Position(&self) -> usize {
        self.position
    }
}

impl Pointer
{
    fn Type() -> StackItemType => StackItemType.Pointer;

    /// <summary>
    /// Create a code pointer with the specified script and position.
    /// </summary>
    /// <param name="script">The <see cref="VM.Script"/> object containing this pointer.</param>
    /// <param name="position">The position of the pointer in the script.</param>
    // fn Pointer(script, position:usize)
    // {
    //     this.Script = script;
    //     this.Position = position;
    // }

    fn Equals(other:&dyn StackItem) -> bool
    {
        if other == this { return true; }

        if (other
        is
        Pointer
        p) return Position == p.Position & &Script == p.Script;
        return false;
    }

    fn GetBoolean() -> bool
    {
        return true;
    }

    fn GetHashCode() -> i32
    {
        return HashCode.Combine(Script, Position);
    }
}
