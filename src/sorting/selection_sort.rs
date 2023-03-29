use super::array::{swap};

fn min_index(arr: &Vec<i32>, from_index: usize, length: usize) -> usize {
    let mut min = arr[from_index];
    let mut index = from_index;
    for i in from_index..length  {
        let value = arr[i];
        if value < min {
            min = value;
            index = i;
        }
        
    }
    return index
}

pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let length = arr.len();
    for i in 0..(length - 1) {
        let min_index = min_index(&arr, i, length);
        arr = swap(arr, i, min_index)
    }
    return arr;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_min_index_when_first_element_is_smallest() {
        let array = vec![1, 4, 2, 3, 1, 5, 121331, 11];
        let length = array.len();
        let subject = min_index(&array, 0, length);
        assert_eq!(0, subject);
    }

    #[test]
    fn test_min_index_when_middle_element_is_smallest() {
        let array = vec![10, 4, 2, 3, 1, 5, 121331, 11];
        let length = array.len();
        let subject = min_index(&array, 4, length);
        assert_eq!(4, subject);
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
