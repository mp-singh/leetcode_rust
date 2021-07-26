pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    let v: Vec<u32> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut index;
    if v.len() == 2 {
        index = (0, 1);
    } else if v.len() % 2 == 0 {
        index = (v.len() / 2 - 1, v.len() / 2);
    } else {
        index = (v.len() / 2 - 1, v.len() / 2 + 1);
    }

    while index.1 < v.len() {
        if v.get(index.0) != v.get(index.1) {
            return false;
        }
        match index.0 != 0 {
            true => index = (index.0 - 1, index.1 + 1),
            false => break,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(is_palindrome(1223221), true)
    }
    #[test]
    fn t2() {
        assert_eq!(is_palindrome(121), true)
    }
    #[test]
    fn t3() {
        assert_eq!(is_palindrome(-121), false)
    }
    #[test]
    fn t4() {
        assert_eq!(is_palindrome(10), false)
    }
    #[test]
    fn t5() {
        assert_eq!(is_palindrome(22), true)
    }
    #[test]
    fn t6() {
        assert_eq!(is_palindrome(1001), true)
    }
}
