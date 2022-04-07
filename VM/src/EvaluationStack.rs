use crate::reference_counter::reference_counter;
use std::option::Iter;
use crate::StackItem::StackItem;
use crate::StackItemType::StackItemType;
use alloc::vec::Vec;
use std::any::TypeId;
use primitive_types::H256;

#[derive(Clone, Debug)]
pub struct EvaluationStack {
    inner_list: Vec<StackItem>,
    reference_counter: reference_counter,

}

impl StackItem for EvaluationStack {
    fn Type() -> StackItemType {
        todo!()
    }

    fn ConvertTo(&self, typ: StackItemType) -> Self {
        todo!()
    }
}

//     /// <summary>
/// Represents the evaluation stack in the VM.
/// </summary>
impl EvaluationStack
{

// internal EvaluationStack(reference_counter reference_counter)
// {
// this.reference_counter = reference_counter;
// }

    /// <summary>
    /// Gets the number of items on the stack.
    /// </summary>
    pub fn count(&self) -> usize { self.inner_list.len() }

    pub fn clear(&mut self)
    {
        for item in self.inner_list {
            self.reference_counter.RemoveStackReference(item);
        }

        self.inner_list.clear();
    }

    pub fn copy_to(&mut self, stack: &EvaluationStack, mut count: Option<i32>)
    {
        if count.is_none() { count = Some(-1); }
        if count.unwrap() < -1 || count.unwrap() > self.inner_list.len() as i32 { panic!(); }
        if count.unwrap() == 0 { return; }
        if count.unwrap() == -1 || count.unwrap() == self.inner_list.len()
        { stack.inner_list.AddRange(&self.inner_list); } else { stack.inner_list.AddRange(&self.inner_list.Skip(self.inner_list.len() - count)); }
    }

    pub fn enumerator(&self) -> std::slice::Iter<'_, _>
    {
        self.inner_list.iter()
    }

    // IEnumerator IEnumerable.GetEnumerator()
    // {
    // return inner_list.GetEnumerator();
    // }

    #[inline]
    pub fn insert(&mut self, index: i32, item: &StackItem)
    {
        if index > self.inner_list.len() as i32 { panic!(); }

        self.inner_list.insert(self.inner_list.len() - index, item);
        self.reference_counter.add_stack_reference(item);
    }

    pub fn moveTo(&self, stack: &EvaluationStack, mut count: Option<i32>)
    {
        if count.is_none() {
            count = Some(-1);
        }
        if count.unwrap() == 0 { return; }

        CopyTo(stack, count);

        if count.unwrap() == -1 || count.unwrap() == self.inner_list.len() as i32
        {
            inner_list.Clear();
        } else {
            self.inner_list.remove_range(inner_list.Count - count, count);
        }
    }

    /// <summary>
    /// Returns the item at the specified index from the top of the stack without removing it.
    /// </summary>
    /// <param name="index">The index of the object from the top of the stack.</param>
    /// <returns>The item at the specified index.</returns>
    pub fn peek(&mut self, mut index: Option<i32>) -> &StackItem
    {
        if index.is_none() {
            index = Some(0);
        }
        let mut id = index.unwrap();

        if id >= self.inner_list.len() { panic!(); }
        if id < 0
        {
            id += self.inner_list.len();
            if id < 0 { panic!(); }
        }
        self.inner_list[self.inner_list.len() - id - 1]
    }

    // StackItem IReadOnlyList < StackItem >.this[int index] => Peek(index);

    /// <summary>
    /// Pushes an item onto the top of the stack.
    /// </summary>
    /// <param name="item">The item to be pushed.</param>
    pub fn push(&mut self, item: Box<StackItem>)
    {
        self.inner_list.push(item);
        self.reference_counter.add_stack_reference(item);
    }

    pub fn reverse(&mut self, n: i32)
    {
        if n < 0 || n > self.inner_list.len() as i32 { panic!(); }
        if n <= 1 {
            return;
        }
        self.inner_list.reverse(self.inner_list.len() - n, n);
    }

    /// <summary>
    /// Removes and returns the item at the top of the stack.
    /// </summary>
    /// <returns>The item removed from the top of the stack.</returns>
    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    // pub fn Pop() -> &StackItem
    // {
    //     Remove < StackItem > (0)
    // }

    /// <summary>
    /// Removes and returns the item at the top of the stack and convert it to the specified type.
    /// </summary>
    /// <typeparam name="T">The type to convert to.</typeparam>
    /// <returns>The item removed from the top of the stack.</returns>
    pub fn pop<T>(&mut self) -> T
        where T: StackItem
    {
        self.remove::<T>(0)
    }

    pub fn remove<T>(&mut self, mut index: i32) -> T
        where T: StackItem
    {
        if index >= self.inner_list.len() as i32 { panic!(); }
        if index < 0
        {
            index += self.inner_list.len();
            if index < 0 { panic!(); }
        }
        index = self.inner_list.len() as i32 - index - 1;
        // if (inner_list[index] is not T item)
        // throw new InvalidCastException( $ "The item can't be casted to type {typeof(T)}");
        // TypeId::of::<T>() == TypeId::of::<String>()
        self.inner_list.remove_at(index);
        self.reference_counter.remove_stack_reference(item);
        item
    }
}
