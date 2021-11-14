pub fn hamming_weight(n: u32) -> i32 {
    n.to_string().chars().filter(|c| c == &'1').count() as i32
}

#[cfg(test)]
mod tests {
    use crate::hamming_weight;

    #[test]
    fn it_works() {
        assert_eq!(hamming_weight(00000000000000000000000000001011), 3);
        assert_eq!(hamming_weight(00000000000000000000000010000000), 1);
    }
}
