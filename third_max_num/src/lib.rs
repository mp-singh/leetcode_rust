pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = vec![];
    let mut j = 0;
    for num in nums.iter() {
        match result.is_empty() {
            true => {
                j += 1;
                result.push(*num)
            }
            false => {
                if num > &result[j-1] {
                    j += 1;
                    result.push(*num)
                }
            }
        }
    }
    *result
        .get(result.len() - 3)
        .unwrap_or_else(|| result.get(result.len() - 1).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(third_max(vec![1, 2, 2, 3, 2, 2, 1]), 1)
    }
    #[test]
    fn t2() {
        assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
    }
    #[test]
    fn t3() {
        assert_eq!(third_max(vec![1, 2]), 2);
    }
    #[test]
    fn t4() {
        assert_eq!(third_max(vec![1, 2, 2, 3, 2, 2, 1]), 3);
    }
    #[test]
    fn t5() {
        assert_eq!(third_max(vec![1, 2, 2, 3, 2, 2, 1]), 3);
    }
    #[test]
    fn t6() {
        assert_eq!(third_max(vec![1, 2, 2, 3, 2, 2, 1]), 3);
    }
}
