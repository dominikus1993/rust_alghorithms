use std::collections::{BTreeMap};

fn count(arr: &Vec<i32>) -> BTreeMap<i32, i32> {
    let mut result:  BTreeMap<i32, i32>  = BTreeMap::new();
    for element in arr.iter() {
        let key = element.clone();
        if result.contains_key(element) {
            result.entry(key).and_modify(|x| *x += 1);
        } else {
            result.insert(key, 1);
        }
    }
    return result
}

pub fn sort(arr: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    result.reserve(arr.len());
    let counted_elements = count(&arr);
    for (key, value) in counted_elements.iter() {
        for _ in 0..*value {
            result.push(key.clone());
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cout() {
        let array = vec![1, 4, 2, 11, 3, 1, 5, 121331];
        let expected = BTreeMap::from([(1, 2), (2, 1), (3, 1), (4, 1), (5, 1), (11, 1), (121331, 1)]);
        let subject = count(&array);
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_bubble_sort() {
        let array = vec![1, 4, 2, 3, 1, 5, 121331, 11];
        let expected = vec![1, 1, 2, 3, 4, 5, 11, 121331];
        let subject = sort(array);
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_small_array_bubble_sort() {
        let array = vec![6, 4];
        let expected = vec![4, 6];
        let subject = sort(array);
        assert_eq!(expected, subject);
    }
}
