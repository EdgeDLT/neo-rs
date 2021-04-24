use std::any::type_name;
use std::collections::VecDeque;

use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;

#[derive[Debug, Copy, Clone, Eq, PartialEq]]
pub struct Struct<T>(pub(crate) [T]);

/// <summary>
/// Represents a structure in the VM.
/// </summary>
impl Struct<T>
{
    const Type: StackItemType = StackItemType.Struct;

    /// <summary>
    /// Create a structure with the specified fields.
    /// </summary>
    /// <param name="fields">The fields to be included in the structure.</param>
    fn Struct(IEnumerable<StackItem> ? fields = null) : this(null, fields)
    {}

    /// <summary>
    /// Create a structure with the specified fields. And make the structure use the specified <see cref="ReferenceCounter"/>.
    /// </summary>
    /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> to be used by this structure.</param>
    /// <param name="fields">The fields to be included in the structure.</param>
    fn Struct(ReferenceCounter? referenceCounter, IEnumerable<StackItem> ? fields = null)
    : base(referenceCounter, fields)
    {}

    /// <summary>
    /// Create a new structure with the same content as this structure. All nested structures will be copied by value.
    /// </summary>
    /// <returns>The copied structure.</returns>
    fn Clone(&self) -> Self
    {
        Struct result = new(ReferenceCounter);
        let mut queue = VecDeque::new();
        queue.push_back(result);
        queue.push_back(self);

        while queue.len() > 0
        {
            let mut a = queue.pop_front().unwrap() as Struct<T>;
            let mut b = queue.pop_front().unwrap() as Struct<T>;
            for (item in b)
            {
                if (item
                is
                Struct
                sb)
                {
                    Struct
                    sa = new(ReferenceCounter);
                    a.Add(sa);
                    queue.Enqueue(sa);
                    queue.Enqueue(sb);
                }
                else
                {
                    a.Add(item);
                }
            }
        }
        return result;
    }

    fn ConvertTo(Type: StackItemType) -> Box<dyn StackItem>
    {
        if (Type == StackItemType.Array)
        return new;
        Array(ReferenceCounter, new List < StackItem > (_array));
        return base.ConvertTo(Type);
    }

    fn Equals(&self, other: &Struct<T>) -> bool
    {
        let mut stack1: VecDeque<dyn StackItem> = VecDeque::new();
        let mut stack2: VecDeque<dyn StackItem> = VecDeque::new();

        stack1.push_back(self.clone());
        stack2.Push(other.clone());
        while stack1.len() > 0
        {
            let mut a = stack1.pop_back().unwrap() as Struct<T>;
            let mut b = stack2.pop_back().unwrap() as Struct<T>;

            if ReferenceEquals(a, b) { continue; }
            if type_name::<b>() == type_name::<Self>() { return false; }
            if a.0.len() != b.0.len() { return false; }
            for ia in a {
                stack1.push_back(ia);
            }
            for ib in b {
                stack2.push_back(ib);
            }
        }
        return true;
    }
}
