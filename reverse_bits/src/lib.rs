pub fn reverse_bits(x: u32) -> u32 {
    //reverse bits
    let mut y = 0;
    let mut j = x;
    for _ in 0..32 {
        y = y << 1;
        y = y | (j & 1);
        j = j >> 1;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        assert_eq!(
            reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
    }
}
