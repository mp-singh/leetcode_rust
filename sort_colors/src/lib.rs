pub fn sort_colors(nums: &mut Vec<i32>) -> &Vec<i32> {
    let red = nums.iter().filter(|&red| *red == 0).count();
    let white = nums.iter().filter(|&white| *white == 1).count();
    let blue = nums.iter().filter(|&blue| *blue == 2).count();

    for item in nums.iter_mut().take(red) {
        *item = 0;
    }
    for item in nums.iter_mut().skip(red).take(white) {
        *item = 1;
    }
    for item in nums.iter_mut().skip(red + white).take(blue) {
        *item = 2;
    }
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
