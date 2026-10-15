use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::with_capacity(nums.len());

        for (i, n) in nums.iter().enumerate() {
            let required = target - n;
            if let Some(r) = map.get(&required) {
                return vec![*r, i as i32];
            }
            map.insert(*n, i as i32);
        }
        vec![]
    }
}
pub fn main() {
    let nums = vec![-1, -2, -3, -4, -5];
    let target = -8;
    println!("{:?}", Solution::two_sum(nums, target));
}
