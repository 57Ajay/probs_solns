use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = HashMap::<i32, i32>::new();
        let mut prefix_sum = 0;
        let mut count = 0;
        prefix.insert(0, 1);
        for num in nums {
            prefix_sum += num;

            if let Some(&freq) = prefix.get(&(prefix_sum - k)) {
                count += freq;
            }

            *prefix.entry(prefix_sum).or_insert(0) += 1;
        }

        count
    }
}

pub fn main() {
    let nums = vec![0, 0, 0, 0, 0];
    println!("{}", Solution::subarray_sum(nums, 0));
}
