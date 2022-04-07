use crate::Types::StackItem::StackItem;
use crate::ReferenceCounter::ReferenceCounter;
use std::collections::HashMap;
use std::slice::Iter;


pub trait CompoundType: StackItem
{
    fn reference_counter(&self) -> Option<ReferenceCounter> { None }
    /// <summary>
    /// Create a new <see cref="CompoundType"/> with the specified reference counter.
    /// </summary>
    /// <param name="referenceCounter">The reference counter to be used.</param>
    // protected CompoundType(ReferenceCounter? referenceCounter)
    // {
    // this.ReferenceCounter = referenceCounter;
    // referenceCounter ?.AddZeroReferred(this);
    // }
    fn New(referenceCounter: Option<ReferenceCounter>)->Self{
        // this.ReferenceCounter = referenceCounter;
        // referenceCounter ?.AddZeroReferred(this);
    }

    /// <summary>
    /// The number of items in this VM object.
    /// </summary>
    fn count(&self) -> i32;

    fn at(&self, index: i32) -> &StackItem;

    fn set(&mut self, index: i32, value: &StackItem);

    fn sub_items(&self) -> Iter<'_, dyn StackItem>;

    fn sub_items_count(&self) -> i32;

    fn clear(&self);

    fn deep_copy(&self, refMap: &HashMap<StackItem, StackItem>) -> StackItem;

    fn boolean(&self) -> bool { true }


    /// <summary>
    /// The operation is not supported. Always throw <see cref="NotSupportedException"/>.
    /// </summary>
    /// <exception cref="NotSupportedException">This method always throws the exception.</exception>
    fn hash_code(&self) -> i32 { panic!() }
}
