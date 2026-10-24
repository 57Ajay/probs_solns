pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num = nums[0];
        for i in 1..nums.len() {
            num ^= nums[i];
        }
        return num;
    }
}

pub fn main() {
    let nums = vec![7, 6, 6, 7, 8];
    println!("{}", Solution::single_number(nums));
}
