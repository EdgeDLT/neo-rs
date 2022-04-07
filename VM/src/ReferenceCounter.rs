use std::collections::{HashMap, HashSet};
use crate::CompoundType::CompoundType;
use crate::StackItem::StackItem;

pub struct Entry
{
    StackReferences: i32,
    ObjectReferences: Option<HashMap<CompoundType, usize>>,
}

#[derive(Debug, Copy, Clone)]
pub struct ReferenceCounter {
    counter: HashMap<CompoundType, Entry>,
    // = new(ReferenceEqualityComparer.Instance);
    zero_referred: HashSet<CompoundType>,
    // = new(ReferenceEqualityComparer.Instance);
    references_count: usize,// = 0;
}

impl Default for ReferenceCounter {
    fn default()->Self{
        Self{
            counter: HashMap::default(),
            zero_referred: HashSet::default(),
            references_count: 0
        }
    }
}

/// <summary>
/// Used for reference counting of objects in the VM.
/// </summary>
impl ReferenceCounter
{

    /// <summary>
    /// Indicates the number of this counter.
    /// </summary>
    pub fn count(&self) -> usize { self.references_count }

    pub fn add_reference(&mut self, referred: &StackItem, parent: &CompoundType)
    {
        self.references_count += 1;

        let mut tracing = self.counter.get(compound);
        if tracing.is_none()
        {
            tracing = Some(Entry::new());
            self.counter.Add(compound, &tracing);
        }

        let mut count: usize = 0;

        if tracing.unwrap().ObjectReferences.is_none()
        {
            tracing = Some(&Entry { ObjectReferences: Some(HashMap::new()), ..*tracing.unwrap() });
            count = 1;
        } else {
            if tracing.unwrap().ObjectReferences.unwrap().contains_key(parent)
            { count += 1; } else { count = 1; }
        }
        tracing.unwrap().ObjectReferences.unwrap()[parent] = count;
    }

    pub fn add_references(&mut self, count: usize)
    {
        self.references_count += count;
    }

    pub fn add_stack_reference(&mut self, referred: Box<dyn StackItem>)
    {
        self.references_count += 1;

        let mut entry = self.counter.get(referred);
        if entry.is_some()
        {
            entry.unwrap().StackReferences.unwrap() += 1;
        } else { self.counter.Add(referred, Entry { StackReferences: 1, ObjectReferences: None }); }

        self.zero_referred.Remove(referred);
    }

    pub fn add_zero_referred(&mut self, item: CompoundType)
    {
        self.zero_referred.Add(item);
    }

    pub fn check_zero_referred(&mut self) -> usize
    {
        while self.zero_referred.len() > 0
        {
            let mut toBeDestroyed: HashSet<CompoundType> = new(ReferenceEqualityComparer.Instance);
            for compound in self.zero_referred
            {
                toBeDestroyedInLoop: HashSet<CompoundType> = new(ReferenceEqualityComparer.Instance);
                toBeChecked: Queue<CompoundType> = new();
                toBeChecked.Enqueue(compound);
                while toBeChecked.Count > 0
                {
                    let c = toBeChecked.Dequeue();
                    let mut entry = self.counter.get(c);
                    if entry?.StackReferences > 0
                    {
                        toBeDestroyedInLoop.clear();
                        break;
                    }
                    toBeDestroyedInLoop.Add(c);

                    if entry.unwrap().ObjectReferences.is_none() { continue; }

                    for (pair in entry.unwrap().ObjectReferences){
                        if (pair.Value > 0 && !toBeDestroyed.Contains(pair.Key) && !toBeDestroyedInLoop.Contains(pair.Key))
                        toBeChecked.Enqueue(pair.Key);
                    }
                }
                if toBeDestroyedInLoop.Count > 0
                { toBeDestroyed.UnionWith(toBeDestroyedInLoop); }
            }

            self.zero_referred.clear();

            for compound in toBeDestroyed.iter()
            {
                counter.Remove(compound);
                references_count -= compound.SubItemsCount;
                for subitem in compound.SubItems.OfType<CompoundType>().iter()
                {
                    if toBeDestroyed.Contains(subitem) continue;
                    let entry = counter[subitem];
                    entry.ObjectReferences!.Remove(compound);
                    if entry.StackReferences == 0
                        self.zero_referred.Add(subitem);
                }
            }
        }
        return references_count;
    }

    pub fn remove_reference(&mut self, referred: &StackItem, parent: &CompoundType)
    {
        self.references_count = self.references_count - 1;

        let mut entry: Entry = counter[referred];
        entry.ObjectReferences.unwrap()[parent] -= 1;
        if entry.StackReferences == 0
        { self.zero_referred.insert(referred); }
    }

    pub fn remove_stack_reference(&mut self, referred: &StackItem)
    {
        self.references_count -= 1;
        let mut re = self.counter.get(referred).unwrap().StackReferences;
        re -= 1;
        if re == 0 { self.zero_referred.insert(referred) }
    }
}
