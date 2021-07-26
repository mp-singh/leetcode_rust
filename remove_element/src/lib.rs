pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut valid_size = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[valid_size] = nums[i];
            valid_size += 1;
        }
    }
    valid_size as i32
}

pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| x != &val);
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2)
    }
    #[test]
    fn t2() {
        assert_eq!(remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5)
    }
    #[test]
    fn t3() {
        assert_eq!(remove_element(&mut vec![3, 3], 3), 0)
    }
    #[test]
    fn t4() {
        assert_eq!(remove_element(&mut vec![], 2), 0)
    }
    #[test]
    fn t5() {
        assert_eq!(remove_element2(&mut vec![3, 2, 2, 3], 3), 2)
    }
    #[test]
    fn t6() {
        assert_eq!(remove_element2(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5)
    }
    #[test]
    fn t7() {
        assert_eq!(remove_element2(&mut vec![3, 3], 3), 0)
    }
    #[test]
    fn t8() {
        assert_eq!(remove_element2(&mut vec![], 2), 0)
    }
}
