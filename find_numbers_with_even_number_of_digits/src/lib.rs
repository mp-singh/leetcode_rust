pub struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for mut num in nums {
            let mut result = 0;
            while num > 0 {
                num /= 10;
                result += 1
            }
            if result % 2 == 0 {
                count += 1
            }
        }
        count
    }
    pub fn find_numbers_optimized(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|num| num.to_string().len() % 2 == 0)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_find_numbers() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
        assert_eq!(
            Solution::find_numbers(vec![437, 315, 322, 431, 686, 264, 442]),
            0
        );
        assert_eq!(
            Solution::find_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            1
        );
        assert_eq!(Solution::find_numbers(vec![]), 0);
        assert_eq!(Solution::find_numbers(vec![1]), 0);
    }
    #[test]
    fn test_find_numbers_optimized() {
        assert_eq!(
            Solution::find_numbers_optimized(vec![12, 345, 2, 6, 7896]),
            2
        );
        assert_eq!(
            Solution::find_numbers_optimized(vec![555, 901, 482, 1771]),
            1
        );
        assert_eq!(
            Solution::find_numbers_optimized(vec![437, 315, 322, 431, 686, 264, 442]),
            0
        );
        assert_eq!(
            Solution::find_numbers_optimized(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            1
        );
        assert_eq!(Solution::find_numbers_optimized(vec![]), 0);
        assert_eq!(Solution::find_numbers_optimized(vec![1]), 0);
    }
}
