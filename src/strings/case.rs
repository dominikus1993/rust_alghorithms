
pub fn upper_case(txt: &str) -> String {
    let mut res = Vec::new();
    res.reserve(txt.len());
    for x in txt.as_bytes() {
        let char_val = *x;
        if char_val >= 97 && char_val <= 122 {
            let new_char = char_val - 32;
            res.push(new_char);
        } else {
            res.push(char_val);
        }
    }

    let result = String::from_utf8(res).unwrap();
    return result;
}

pub fn lower_case(txt: &str) -> String {
    let mut res = Vec::new();
    res.reserve(txt.len());
    for x in txt.as_bytes() {
        let char_val = *x;
        if char_val >= 65 && char_val <= 90 {
            let new_char = char_val + 32;
            res.push(new_char);
        } else {
            res.push(char_val);
        }
    }

    let result = String::from_utf8(res).unwrap();
    return result;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_upper_case() {
        let array = "tEst2";
        let subject = upper_case(array);
        assert_eq!("TEST2", subject);
    }

    #[test]
    fn test_lower_case() {
        let array = "tEsT5";
        let subject = lower_case(array);
        assert_eq!("test5", subject);
    }
}