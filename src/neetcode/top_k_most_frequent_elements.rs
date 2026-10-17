use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::with_capacity(nums.len());
        for &n in &nums {
            map.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut store = vec![Vec::<i32>::new(); nums.len() + 1];
        for (k, v) in map {
            store[v].push(k);
        }
        let mut res = Vec::<i32>::with_capacity(k as usize);

        for bucket in store.into_iter().rev() {
            for num in bucket {
                res.push(num);

                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }
}

pub fn main() {
    let nums = vec![7, 7];
    let k = 1;
    println!("{:?}", Solution::top_k_frequent(nums, k));
}
