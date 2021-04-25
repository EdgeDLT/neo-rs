use crate::ReferenceCounter::ReferenceCounter;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Slot {
    referenceCounter: ReferenceCounter,
    items: Vec<StackItem>,
}

/// <summary>
/// Used to store local variables, arguments and static fields in the VM.
/// </summary>
impl Slot
{
    /// <summary>
    /// Gets the item at the specified index in the slot.
    /// </summary>
    /// <param name="index">The zero-based index of the item to get.</param>
    /// <returns>The item at the specified index in the slot.</returns>
    pub fn At(&self, index: i32) -> &StackItem
    {
        self.items[index]
    }

    fn set
    {
        ref var
        oldValue = ref items[index];
        referenceCounter.RemoveStackReference(oldValue);
        oldValue = value;
        referenceCounter.AddStackReference(value);
    }


    /// <summary>
    /// Gets the number of items in the slot.
    /// </summary>
    pub fn Count(&self) -> usize { self.items.len() }

    /// <summary>
    /// Creates a slot containing the specified items.
    /// </summary>
    /// <param name="items">The items to be contained.</param>
    /// <param name="referenceCounter">The reference counter to be used.</param>
    pub fn fromStackItems(items: Vec<StackItem>, referenceCounter: ReferenceCounter) -> Self
    {
        let mut slot = Self {
            referenceCounter,
            items: items.clone(),
        };

        for (item in items)
        {
            slot.referenceCounter.AddStackReference(item);
        }

        slot
    }

    /// <summary>
    /// Create a slot of the specified size.
    /// </summary>
    /// <param name="count">Indicates the number of items contained in the slot.</param>
    /// <param name="referenceCounter">The reference counter to be used.</param>
    public Slot(int count, ReferenceCounter referenceCounter)
    {
    this.referenceCounter = referenceCounter;
    this.items = new StackItem[count];
    System.Array.Fill(items, StackItem.Null);
    referenceCounter.AddReferences(count);
    }

    fn ClearReferences(&mut self)
    {
        for (item in self.items)
        { self.referenceCounter.RemoveStackReference(item); }
    }

    IEnumerator < StackItem > IEnumerable < StackItem >.GetEnumerator()
    {
    foreach (StackItem item in items) yield return item;
    }

// IEnumerator IEnumerable.GetEnumerator()
// {
// return items.GetEnumerator();
// }
}