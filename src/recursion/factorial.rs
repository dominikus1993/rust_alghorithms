pub fn factorial(n: i64) -> i64 {
    match n {
        0 => 1,
        n => n * factorial(n - 1)
    }
}

pub fn factorial_tmp(n: i64, tmp: i64) -> i64 {
    match n {
        0 => tmp,
        n => factorial_tmp(n - 1, n * tmp)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_factorial() {
        let array = vec![(0, 1), (1, 1), (2, 2), (3, 6), (4, 24), (5, 120), (6, 720)];
        for (n, expected) in array {
            let subject = factorial(n);
            assert_eq!(expected, subject);
        }
    }

    #[test]
    fn test_factorial_tmp() {
        let array = vec![(0, 1), (1, 1), (2, 2), (3, 6), (4, 24), (5, 120), (6, 720)];
        for (n, expected) in array {
            let subject = factorial_tmp(n, 1);
            assert_eq!(expected, subject);
        }
    }
}
