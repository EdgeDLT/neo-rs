use std::collections::HashMap;

use crate::ReferenceCounter::ReferenceCounter;
use crate::Types::CompoundType::CompoundType;
use crate::Types::StackItem::StackItem;
use crate::Types::StackItemType::StackItemType;
use std::ptr::null;
use std::slice::Iter;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Array {
    _array: Vec<dyn StackItem>,
    ReferenceCounter: Option<ReferenceCounter>,
}


impl StackItem for Array {
    fn Type() -> StackItemType {
        StackItemType::Array
    }

    fn ConvertTo(&self, typ: StackItemType) -> Box<StackItem>
    {
        if Type == StackItemType::Array && Type == StackItemType::Struct {
            return Struct { ReferenceCounter: self.ReferenceCounter, new List < StackItem > (_array) };
        }
        // return base.ConvertTo( type );

        if typ == Type { return Array { _array: self._array.clone(), ReferenceCounter: self.ReferenceCounter }; }

        if typ == StackItemType::Boolean
        {
            return GetBoolean();
        }
        panic!();
    }
}

impl CompoundType for Array {
    fn ReferenceCounter(&self) -> Option<ReferenceCounter> {
        todo!()
    }

    fn Count(&self) -> i32 {
        self._array.len() as i32
    }

    fn At(&self, index: i32) -> &StackItem
    {
        self._array[index]
    }

    fn Set(&mut self, index: i32, value: &StackItem) {
        ReferenceCounter..RemoveReference(self._array[index], self);
        self._array[index] = value;
        ReferenceCounter?.AddReference(value, this);
    }

    fn SubItems(&self) -> Iter<'_, dyn StackItem> { self._array.iter() }


    fn SubItemsCount(&self) -> i32 { self._array.len() as i32 }

    fn Clear(&mut self) {
        if self.ReferenceCounter != null {
            for item in self._array {
                self.ReferenceCounter.RemoveReference(item, this);
                self._array.clear();
            }
        }
    }

    fn DeepCopy(&self, refMap: &HashMap<dyn StackItem, dyn StackItem>) -> Box<dyn StackItem> {
        if refMap.TryGetValue(this, out StackItem? mappedItem) { return mappedItem; }

        let mut result = this
        is
        Struct?
        new
        Struct(ReferenceCounter): new
        Array(ReferenceCounter);
        refMap.Add(this, result);
        for (item in self._array){
            result.Add(item.DeepCopy(refMap));
        }
        return result;
    }
}

/// <summary>
/// Represents an array or a complex object in the VM.
/// </summary>
impl Array
{
    /// <summary>
    /// Create an array containing the specified items. And make the array use the specified <see cref="ReferenceCounter"/>.
    /// </summary>
    /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> to be used by this array.</param>
    /// <param name="items">The items to be included in the array.</param>
    // pub fn Array(ReferenceCounter? referenceCounter, IEnumerable<StackItem> ? items = null)
    // : base(referenceCounter)
    // {
    // _array = items switch
    // {
    // null => new List < StackItem > (),
    // List < StackItem > list => list,
    // _ => new List < StackItem > (items)
    // };
    // if (referenceCounter != null)
    // foreach (StackItem item in _array)
    // referenceCounter.AddReference(item, this);
    // }

    /// <summary>
    /// Add a new item at the end of the array.
    /// </summary>
    /// <param name="item">The item to be added.</param>
    pub fn Add(&mut self, item: &StackItem)
    {
        self._array.push(item);
        self.ReferenceCounter?.AddReference(item, this);
    }


    pub fn GetEnumerator(&self) -> Iter<'_, dyn StackItem>
    {
        self._array.iter()
    }

    pub fn RemoveAt(&mut self, index: i32)
    {
        self.ReferenceCounter?.RemoveReference(_array[index], this);
        _array.RemoveAt(index);
    }

    pub fn Reverse(&mut self)
    {
        self._array.reserve(0);
    }
}

