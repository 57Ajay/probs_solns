pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && (nums[i] as usize) <= n {
                let target = nums[i] as usize - 1;

                if nums[target] == nums[i] {
                    break;
                }

                nums.swap(i, target);
            }
        }

        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
    }
}

pub fn main() {
    let nums = vec![-2, -1, 0];
    println!("{}", Solution::first_missing_positive(nums));
}
