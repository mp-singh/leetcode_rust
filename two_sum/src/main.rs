use std::collections::HashMap;

fn main() {
    println!("Two Sum Problem. Run the tests to see if they pass using 'cargo test'");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut compliments: HashMap<i32, i32> = HashMap::new();
    for (k, v) in nums.iter().enumerate() {
        match compliments.get(&(target - v)) {
            Some(d) => return vec![*d, k as i32],
            None => compliments.insert(*v, k as i32),
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1])
    }
    #[test]
    fn t2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), [1, 2])
    }
    #[test]
    fn t3() {
        assert_eq!(two_sum(vec![3, 3], 6), [0, 1])
    }
}
