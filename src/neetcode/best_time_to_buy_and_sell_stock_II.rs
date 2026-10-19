pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i - 1] < prices[i] {
                profit += prices[i] - prices[i - 1];
            }
        }

        profit
    }
}

pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(prices));
}
