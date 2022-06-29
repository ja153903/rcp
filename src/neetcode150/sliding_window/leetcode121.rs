#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut min_buy = prices[0];
        let mut profit: i32 = 0;

        for &price in prices.iter().skip(1) {
            min_buy = min_buy.min(price);
            profit = profit.max(price - min_buy);
        }

        profit
    }
}
