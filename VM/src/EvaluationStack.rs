use crate::ReferenceCounter::ReferenceCounter;
use std::option::Iter;

pub struct EvaluationStack {
    innerList: Vec<StackItem>,
    referenceCounter: ReferenceCounter,

}

impl StackItem for EvaluationStack {}

//     /// <summary>
/// Represents the evaluation stack in the VM.
/// </summary>
impl EvaluationStack
{

// internal EvaluationStack(ReferenceCounter referenceCounter)
// {
// this.referenceCounter = referenceCounter;
// }

    /// <summary>
    /// Gets the number of items on the stack.
    /// </summary>
    pub fn Count(&self) -> usize { self.innerList.len() }

    pub fn Clear(&mut self)
    {
        for item in self.innerList {
            self.referenceCounter.RemoveStackReference(item);
        }

        self.innerList.clear();
    }

    pub fn CopyTo(&mut self, stack: &EvaluationStack, mut count: Option<i32>)
    {
        if count.is_none() { count = Some(-1); }
        if count.unwrap() < -1 || count.unwrap() > self.innerList.len() as i32 { panic!(); }
        if count.unwrap() == 0 { return; }
        if count.unwrap() == -1 || count.unwrap() == self.innerList.len()
        { stack.innerList.AddRange(&self.innerList); } else { stack.innerList.AddRange(&self.innerList.Skip(self.innerList.len() - count)); }
    }

    pub fn GetEnumerator(&self) -> std::slice::Iter<'_, _>
    {
        self.innerList.iter()
    }

    // IEnumerator IEnumerable.GetEnumerator()
    // {
    // return innerList.GetEnumerator();
    // }

    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    pub fn Insert(&mut self, index: i32, item: &StackItem)
    {
        if index > self.innerList.len() as i32 { panic!(); }
        // throw
        // new
        // InvalidOperationException($ "Insert out of bounds: {index}/{innerList.Count}");
        self.innerList.Insert(innerList.Count - index, item);
        self.referenceCounter.AddStackReference(item);
    }

    pub fn MoveTo(&self, stack: &EvaluationStack, mut count: Option<i32>)
    {
        if count.is_none() {
            count = Some(-1);
        }
        if count.unwrap() == 0 { return; }

        CopyTo(stack, count);

        if count.unwrap() == -1 || count.unwrap() == self.innerList.len() as i32
        {
            innerList.Clear();
        } else {
            self.innerList.RemoveRange(innerList.Count - count, count);
        }
    }

    /// <summary>
    /// Returns the item at the specified index from the top of the stack without removing it.
    /// </summary>
    /// <param name="index">The index of the object from the top of the stack.</param>
    /// <returns>The item at the specified index.</returns>
    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    pub fn Peek(&mut self, mut index: Option<i32>) -> &StackItem
    {
        if index.is_none() {
            index = Some(0);
        }
        let mut id = index.unwrap();

        if id >= self.innerList.Count { panic!(); }
        // throw?rationException($ "Peek out of bounds: {index}/{innerList.Count}");
        if id < 0
        {
            id += self.innerList.len();
            if id < 0 { panic!(); }
            // throw
            // new
            // InvalidOperationException($ "Peek out of bounds: {index}/{innerList.Count}");
        }
        self.innerList[self.innerList.len() - id - 1]
    }

    // StackItem IReadOnlyList < StackItem >.this[int index] => Peek(index);

    /// <summary>
    /// Pushes an item onto the top of the stack.
    /// </summary>
    /// <param name="item">The item to be pushed.</param>
    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    pub fn Push(&mut self, item: &StackItem)
    {
        self.innerList.push(item);
        self.referenceCounter.AddStackReference(item);
    }

    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    pub fn Reverse(&mut self, n: i32)
    {
        if n < 0 || n > self.innerList.len() as i32 { panic!(); }
        // throw new ArgumentOutOfRangeException(nameof(n));
        if n <= 1 {
            return;
        }

        self.innerList.Reverse(self.innerList.len() - n, n);
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
    // [MethodImpl(MethodImplOptions.AggressiveInlining)]
    pub fn Pop<T>(&mut self) -> T
        where T: StackItem
    {
        self.Remove::<T>(0)
    }

    pub fn Remove<T>(&mut self, mut index: i32) -> T
        where T: StackItem
    {
        if index >= self.innerList.len() as i32 { panic!(); }
        // throw new ArgumentOutOfRangeException(nameof(index));
        if index < 0
        {
            index += self.innerList.len();
            if index < 0 { panic!(); }
            // throw new ArgumentOutOfRangeException(nameof(index));
        }
        index = self.innerList.len() as i32 - index - 1;
        // if (innerList[index] is not T item)
        // throw new InvalidCastException( $ "The item can't be casted to type {typeof(T)}");
        self.innerList.RemoveAt(index);
        self.referenceCounter.RemoveStackReference(item);
        item
    }
}
