pub fn max_profit_brute_force(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;

    for (i, val) in prices.iter().enumerate() {
        let j = i + 1;
        if j < prices.len() && val > prices.get(j).unwrap() {
            continue;
        }
        for index in j..prices.len() {
            if prices.get(index).unwrap() - val > max_profit {
                max_profit = prices.get(index).unwrap() - val;
            }
        }
    }
    max_profit
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min = i32::MAX;

    prices.iter().for_each(|p| {
        if p < &min {
            min = *p;
        } else {
            max_profit = i32::max(max_profit, p - min)
        }
    });
    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(max_profit(vec![1, 5, 3, 6]), 5)
    }
    #[test]
    fn t2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
    #[test]
    fn t3() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
