use std::ops::Add;

extern crate num_bigint;
use num_bigint::BigInt;

fn main() {
    println!("Plus One Problem. Run the tests to see if they pass using 'cargo test'");
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    digits
        .into_iter()
        .map(|v| v.to_string())
        .collect::<String>()
        .parse::<BigInt>()
        .unwrap()
        .add(BigInt::from(1))
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|d| d as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(
            plus_one(vec![
                7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9,
                4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 6
            ]),
            [
                7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9,
                4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 7
            ]
        )
    }

    #[test]
    fn t2() {
        assert_eq!(plus_one(vec![1,2,3]),[1,2,4])
    }

    #[test]
    fn t3() {
        assert_eq!(plus_one(vec![4,3,2,1]),[4,3,2,2])
    }
}
