use std::collections::HashMap;

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut freq = nums
        .iter()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
        .into_iter()
        .collect::<Vec<_>>();

    freq.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0).reverse()));
    let res = freq.iter().fold(Vec::new(), |mut r, t| {
        for _ in 0..t.1 {
            r.push(*t.0);
        }
        r
    });
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
        assert_eq!(frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
    }
}
