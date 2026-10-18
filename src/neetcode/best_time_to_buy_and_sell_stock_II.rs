pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut buy = prices[0];
        let mut sell = 0;
        for p in 1..prices.len() - 1 {
            if prices[p] < buy {
                buy = prices[p] as i32;
            }

            if prices[p + 1] < prices[p] {
                sell = prices[p];
                buy = i32::MAX;
            }

            profit += sell - buy;
        }

        profit
    }
}

pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(prices));
}
