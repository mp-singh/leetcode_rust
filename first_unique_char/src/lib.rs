use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut char_count = HashMap::new();
    s.chars().for_each(|c| {
        if char_count.contains_key(&c) {
            *char_count.get_mut(&c).unwrap() += 1;
        } else {
            char_count.insert(c, 1);
        }
    });
    s.chars()
        .enumerate()
        .find(|(_, c)| *char_count.get(c).unwrap() == 1)
        .map(|(i, _)| i as i32)
        .unwrap_or(-1)
}
#[cfg(test)]
mod tests {
    use crate::first_uniq_char;

    #[test]
    fn it_works() {
        assert_eq!(first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(first_uniq_char("aabbccddeeffgghh".to_string()), -1);
    }
}
