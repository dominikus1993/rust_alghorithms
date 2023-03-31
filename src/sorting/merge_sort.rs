use super::array::swap;

fn merge(arr: Vec<i32>, from: usize, mid: usize, to: usize) -> Vec<i32> {
    println!("{} {} {}", from, mid, to);
    let mut res: Vec<i32> = Vec::new();
    res.reserve(arr.len());
    for index in 0..from {
        res.push(arr[index]);
    }

    let mut i = from;
    let mut j = mid;

    while i < mid && j < to {
        if arr[j] >= arr[i] {
            res.push(arr[i]);
            i = i + 1;
        } else {
            res.push(arr[j]);
            j = j + 1;
        }
    }
    while i <= mid {
        res.push(arr[i]);
        i = i + 1;
    }
    while j < to {
        res.push(arr[j]);
        j = j + 1;
    }

    for index in to..arr.len() {
        res.push(arr[index]);
    }
    res
}

fn sort_internal(mut arr: Vec<i32>, from: usize, to: usize) -> Vec<i32> {
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
        let array = vec![6, 4, 1];
        let expected = vec![1, 4, 6];
        let subject = sort(array);
        assert_eq!(expected, subject);
    }
}
