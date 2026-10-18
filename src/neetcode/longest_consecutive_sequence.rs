use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut set = HashSet::<i32>::with_capacity(nums.len());

        for &num in &nums {
            set.insert(num);
        }

        let mut max = 0;

        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut current = num;
                let mut count = 1;

                while set.contains(&(current + 1)) {
                    current += 1;
                    count += 1;
                }

                max = max.max(count);
            }
        }

        max
    }
}

pub fn main() {
    let nums = vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6];
    println!("{}", Solution::longest_consecutive(nums));
}
