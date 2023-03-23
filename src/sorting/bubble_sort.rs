

pub fn sort(mut arr: Vec<i32>) -> Vec<i32> {
     
     return arr;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_format_empty_board() {
        let mut array = vec![1, 4, 2, 3,1,5, 121331, 11];
        let expected = vec![1, 1, 2, 3, 4, 5, 11, 121331];
        let subject = sort(array);
        assert_eq!(expected, subject);
    }
}