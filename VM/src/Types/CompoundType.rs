use crate::Types::StackItem::StackItem;
use crate::ReferenceCounter::ReferenceCounter;
use std::collections::HashMap;
use std::slice::Iter;


pub trait CompoundType: StackItem
{
    fn ReferenceCounter(&self) -> Option<ReferenceCounter> { None }
    /// <summary>
    /// Create a new <see cref="CompoundType"/> with the specified reference counter.
    /// </summary>
    /// <param name="referenceCounter">The reference counter to be used.</param>
    // protected CompoundType(ReferenceCounter? referenceCounter)
    // {
    // this.ReferenceCounter = referenceCounter;
    // referenceCounter ?.AddZeroReferred(this);
    // }

    /// <summary>
    /// The number of items in this VM object.
    /// </summary>
    fn Count(&self) -> i32;

    fn At(&self, index: i32) -> &StackItem;

    fn Set(&mut self, index: i32, value: &StackItem);

    fn SubItems(&self) -> Iter<'_, dyn StackItem>;

    fn SubItemsCount(&self) -> i32;

    fn Clear(&self);

    fn DeepCopy(&self, refMap: &HashMap<StackItem, StackItem>) -> StackItem;

    fn GetBoolean(&self) -> bool { true }


    /// <summary>
    /// The operation is not supported. Always throw <see cref="NotSupportedException"/>.
    /// </summary>
    /// <exception cref="NotSupportedException">This method always throws the exception.</exception>
    fn GetHashCode(&self) -> i32 { panic!() }
}
