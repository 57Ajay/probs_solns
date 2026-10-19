use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(2);
        let one_third = nums.len() / 3;
        let mut map = HashMap::<i32, usize>::with_capacity(nums.len());
        for num in nums {
            map.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        for (k, v) in map {
            if v > one_third {
                res.push(k);
            }
            if res.len() == 2 {
                break;
            }
        }

        res
    }
}

pub fn main() {
    let nums = vec![5, 2, 3, 2, 2, 2, 2, 5, 5, 5];
    println!("{:?}", Solution::majority_element(nums));
}
