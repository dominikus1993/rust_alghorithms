
fn divide_point(left: usize, right: usize) -> usize {
    (left + (right-left)) / 2
}

fn swap(mut arr: Vec<i32>, index: usize, index2: usize) ->  Vec<i32> {
    if index == index2 {
        return arr;
    }
    let tmp = arr[index];
    arr[index] = arr[index2];
    arr[index2] = tmp;
    arr
}

fn divide_and_sort(mut arr: Vec<i32>, left: usize, right: usize) -> (Vec<i32>, usize) {
    let pivot = divide_point(left, right);
    let pivot_element = arr[pivot];
    let mut result = left;
    arr = swap(arr, pivot, right);

    for i in left..(right - 1) {
        if arr[i] < pivot_element {
            arr = swap(arr, i, result);
            result = result + 1;
        }
    }
    arr = swap(arr, result, right);
    (arr, result)
}

fn sort(mut arr: Vec<i32>,left: usize, right: usize) -> Vec<i32> {
    let (mut arr, position) = divide_and_sort(arr, left, right);
    arr = sort(arr, left, position - 1);
    arr = sort(arr, position + 1, right);
    return arr;
}

pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let length = arr.len();
    return sort(arr, 0, length - 1);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_divide_point_when_number_of_elements_is_even() {
        let left = 0;
        let right = 10;
        let subject = divide_point(left, right);
        assert_eq!(5, subject);
    }

    #[test]
    fn test_divide_point_when_number_of_elements_is_odd() {
        let left = 0;
        let right = 9;
        let subject = divide_point(left, right);
        assert_eq!(4, subject);
    }

    #[test]
    fn test_divide_point() {
        let left = 0;
        let right = 9;
        let subject = divide_point(left, right);
        assert_eq!(5, subject);
    }

    #[test]
    fn test_bubble_sort() {
        let array = vec![1, 4, 2, 3, 1, 5, 121331, 11];
        let expected = vec![1, 1, 2, 3, 4, 5, 11, 121331];
        let subject = quick_sort(array);
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_small_array_bubble_sort() {
        let array = vec![6, 4];
        let expected = vec![4, 6];
        let subject = quick_sort(array);
        assert_eq!(expected, subject);
    }
}
