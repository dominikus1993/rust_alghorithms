use super::array::{swap};

fn merge(mut arr: Vec<i32>, from: usize, mid: usize, to: usize) -> Vec<i32> {
    let mut i = from;
    let mut j = mid;
    loop {
        if arr[i] > arr[j] {
           arr = swap(arr, i, j);
        }
    }
    arr
}

fn sort_internal(mut arr: Vec<i32>, from: usize, to: usize) -> Vec<i32>  {
    if to > from {
        let mid = (from + to) / 2;
        arr = sort_internal(arr, from, mid);
        arr = sort_internal(arr, mid + 1, to);
        arr = merge(arr, from, mid, to)
    }
    arr
}

pub fn sort(arr: Vec<i32>) -> Vec<i32> {
    let length = arr.len();
    return sort_internal(arr, 0, length);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_insert_sort() {
        let array = vec![1, 4, 2, 3, 1, 5, 121331, 11];
        let expected = vec![1, 1, 2, 3, 4, 5, 11, 121331];
        let subject = sort(array);
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_small_array_insert_sort() {
        let array = vec![6, 4];
        let expected = vec![4, 6];
        let subject = sort(array);
        assert_eq!(expected, subject);
    }
}
