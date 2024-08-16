pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_int = u128::from_str_radix(&a, 2).unwrap();
        let b_int = u128::from_str_radix(&b, 2).unwrap();
        let binding = a_int + b_int;
        format!("{binding:b}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        )
    }
}
