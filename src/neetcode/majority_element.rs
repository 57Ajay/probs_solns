use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        let mut max = 0;
        let mut max_val = 0;
        for v in nums {
            let x = map.entry(v).and_modify(|e| *e += 1).or_insert(1);
            if *x > max {
                max = *x;
                max_val = v;
            }
        }
        max_val
    }

    pub fn majority_element_o1_space(nums: Vec<i32>) -> i32 {
        // Boyer–Moore Voting Algorithm
        let mut candidate = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                candidate = num;
            }

            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
            println!("candidate: {candidate}");
        }

        candidate
    }
}

pub fn main() {
    println!(
        "{:?}",
        Solution::majority_element_o1_space(vec![12, 11, 33, 11, 11])
    );
}
