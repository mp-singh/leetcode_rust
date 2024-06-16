pub fn can_construct(ransom_note: String, mut magazine: String) -> bool {
    for c in ransom_note.chars() {
        if let Some(i) = magazine.find(c) {
            magazine.remove(i);
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_construct() {
        assert!(!can_construct("a".to_string(), "b".to_string()));
        assert!(!can_construct("aa".to_string(), "ab".to_string()));
        assert!(can_construct("aa".to_string(), "aab".to_string()));
    }
}
