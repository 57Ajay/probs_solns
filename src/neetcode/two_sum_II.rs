use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i < j {
            match (numbers[i] + numbers[j]).cmp(&target) {
                Ordering::Equal => return vec![i as i32 + 1, j as i32 + 1],
                Ordering::Greater => j -= 1,
                Ordering::Less => i += 1,
            }
        }
        vec![]
    }
}

pub fn main() {
    let numbers = vec![-1, 0];
    let target = -1;
    let res = Solution::two_sum(numbers, target);
    println!("{:?}", res);
}
