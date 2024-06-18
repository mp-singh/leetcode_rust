// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(maximum, count), &num| {
                if num == 1 {
                    (maximum.max(count + 1), count + 1)
                } else {
                    (maximum, 0)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 1, 1, 1, 1]),
            6
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![0, 0, 0, 0, 0, 0]),
            0
        );
    }
}
