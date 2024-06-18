pub struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut f = vec![0; 128];
        for b in s.bytes() {
            f[b as usize] += 1
        }
        let mut cs: Vec<_> = s.chars().collect();
        cs.sort_unstable_by_key(|&c| (-f[c as usize], c));
        cs.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        assert_eq!(
            Solution::frequency_sort("Aabb".to_string()),
            "bbAa".to_string()
        );
    }
}
