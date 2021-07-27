pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = vec![];
    let mut j = 0;
    let mut n = nums;
    let k = 3;
    n.sort_unstable();

    for num in n.iter() {
        match result.is_empty() {
            true => {
                result.push(*num);
                j += 1;
            }
            false => {
                if num > &result[j - 1] {
                    result.push(*num);
                    j += 1;
                }
            }
        }
    }

    match result.len() >= k {
        true => result[result.len() - k],
        false => result[result.len() - 1],
    }
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
        assert_eq!(third_max(vec![3, 2, 1]), 1);
    }
    #[test]
    fn t3() {
        assert_eq!(third_max(vec![1, 2]), 2);
    }
    #[test]
    fn t4() {
        assert_eq!(third_max(vec![1, 2, 2, 3, 2, 2, 1]), 1);
    }
}
