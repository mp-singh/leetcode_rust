use std::{collections::HashMap, iter::FromIterator};

pub fn frequency_sort(s: String) -> String {
    let mut hash_vec = s
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
        .into_iter()
        .collect::<Vec<_>>();

    hash_vec.sort_by(|a, b| b.1.cmp(&a.1));

    String::from_iter(
        hash_vec
            .iter()
            .map(|(c, times)| c.to_string().repeat(*times)),
    )
}

#[cfg(test)]
mod tests {
    use crate::frequency_sort;

    #[test]
    fn it_works() {
        assert_eq!(frequency_sort("Aabb".to_string()), "bbAa".to_string());
    }
}
