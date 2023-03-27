use super::array::{swap};

pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let length = arr.len();
    for i in 1..length {
        let elemnt = arr[i];
        for j in 0..i {
            if arr[j] > elemnt {
                arr = swap(arr, i, j)
            }
        } 
    }
    return arr;
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
