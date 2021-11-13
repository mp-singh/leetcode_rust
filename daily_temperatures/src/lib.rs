pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; temperatures.len()];
    for i in 0..temperatures.len() {
        let mut j = i + 1;
        while j < temperatures.len() {
            if temperatures[j] > temperatures[i] {
                result[i] = (j - i) as i32;
                break;
            }
            j += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::daily_temperatures;

    #[test]
    fn test_daily_temps() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60,]), vec![1, 1, 1, 0]);
    }
}
