use std::collections::HashMap;

pub trait NeonObject<T, M> {
    fn export() -> M;
    fn equals<T>(other: T) -> bool;
}


pub fn compare_neon_object_array<'a, T, M, N>(arr1: &'a [NeonObject<T, M>], arr2: &'a [N]) -> bool {
    if arr1.len() != arr2.len() {
        false
    }

    for c in arr1 {

        // TODO:

        // arr2
        // if !arr2.find((cl) => c.equals(c)) {
        //     false
        // }
    }
    true
}

pub fn compare_object<T>(current: &HashMap<String, T>, other: &HashMap<String, T>) -> bool {
    let keys = current.keys();

    let otherKeys = other.keys();

    if keys.len() != otherKeys.len() {
        false
    }

    for key in keys {

        if other.contains_key(key) && current[key] == other[key] {
            continue;
        }
        false
    }
    true
}

pub fn compare_unsorted_plain_arrays<'a, N, M>(current: &'a [N], other: &'a [M]) -> bool {
    if current.len() != other.len() {
        false
    }
    for i in 0..current.len() {
        if current[i] != other[i] {
            false
        }
    }
    true
}

pub fn compare_array<'a, N, M>(current: &'a [N], other: &'a [M]) -> bool {
    if current.len() != other.len() {
        false
    }

    for i in 0..current.len() {
        if !compare_object(&current[i], &other[i]) {
            false
        }

        if current[i] != other[i] {
            false
        }
    }
    true
}
