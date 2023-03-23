pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
    let length = arr.len();
    for i in 0..(length - 1) {
        for j in 0..((length - 1) - i) {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
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
