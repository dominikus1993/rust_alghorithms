pub fn fib(n: i64) -> i64 {
    match n {
        0 | 1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

pub fn fib_t(n: i64, a: i64, b: i64) -> i64 {
    match n {
        0 => a,
        1 => b,
        n => fib_t(n - 1, b, a + b)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fib() {
        let array = vec![(0, 1), (1, 1), (2, 2), (3, 3), (4, 5), (5, 8), (6, 13)];
        for (n, expected) in array {
            let subject = fib(n);
            assert_eq!(expected, subject);
        }
    }

    
    #[test]
    fn test_fib_t() {
        let array = vec![(0, 1), (1, 1), (2, 2), (3, 3), (4, 5), (5, 8), (6, 13)];
        for (n, expected) in array {
            let subject = fib_t(n, 1, 1);
            assert_eq!(expected, subject);
        }
    }
}
