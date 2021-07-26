pub fn reverse(x: i32) -> i32 {
    let mut rev = 0;
    let mut curr;
    let mut x = x;
    let mut prev = 0;
    while x != 0 {
        curr = x % 10;
        x /= 10;
        rev = rev * 10 + curr;
        if (rev - curr) / 10 != prev {
            return 0;
        };
        prev = rev;
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(reverse(123), 321)
    }
    #[test]
    fn t2() {
        assert_eq!(reverse(-321), -123)
    }
    #[test]
    fn t3() {
        assert_eq!(reverse(120), 21)
    }
}
