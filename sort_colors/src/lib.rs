pub fn sort_colors(nums: &mut Vec<i32>) -> &Vec<i32> {
    let red = nums.iter().filter(|&red| *red == 0).count();
    let white = nums.iter().filter(|&white| *white == 1).count();
    let blue = nums.iter().filter(|&blue| *blue == 2).count();

    for i in 0..red {
        nums[i] = 0;
    }
    for i in red..(red + white) {
        nums[i] = 1;
    }
    for i in (red + white)..(red + white + blue) {
        nums[i] = 2;
    }
    println!("{:?}", nums);
    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            sort_colors(&mut vec![2, 0, 2, 1, 1, 0]),
            &vec![0, 0, 1, 1, 2, 2]
        );
    }
}
