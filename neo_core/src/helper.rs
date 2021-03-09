use std::collections::HashMap;

pub trait NeonObject<T, M> {
    fn export() -> M;
    fn equals(other: T) -> bool;
}


pub fn compare_neon_object_array<'a, T, M, N>(arr1: &'a [T], arr2: &'a [N]) -> bool {
    if arr1.len() != arr2.len() {
        return false;
    };

    for c in arr1 {

        // TODO:

        // arr2
        // if !arr2.find((cl) => c.equals(c)) {
        //     false
        // }
    }
    true
}

pub fn compare_object<T>(current: &HashMap<String, T>, other: &HashMap<String, T>) -> bool
    where T: Eq {
    let keys = current.keys();

    let other_keys = other.keys();

    if keys.len() != other_keys.len() {
        return false;
    }

    for key in keys {
        if other.contains_key(key) && current[key] == other[key] {
            continue;
        }
        return false;
    }
    true
}

pub fn compare_unsorted_plain_arrays<'a, T>(current: &'a [T], other: &'a [T]) -> bool
    where T: PartialEq {
    if current.len() != other.len() {
        return false;
    }
    for i in 0..current.len() {
        if current[i] != other[i] {
            return false;
        }
    }
    true
}

pub fn compare_array<'a, T>(current: &'a [T], other: &'a [T]) -> bool
where T: PartialEq {
    if current.len() != other.len() {
        return false;
    }

    //TODO:

    // for i in 0..current.len() {
    //
    //     if !compare_object(&current[i], &other[i]) || current[i] != other[i] {
    //         return false;
    //     }
    // }
    true
}
