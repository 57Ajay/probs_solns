pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];
        let mut sum = 0;

        for num in nums {
            if num > max {
                max = num;
            } else if num < min {
                min = num;
            }

            sum += num;
        }
        let act_sum = (max + 1) * max / 2;

        if act_sum == sum && min == 0 {
            return max + 1;
        } else {
            return act_sum - sum;
        }
    }

    pub fn missing_number_xor(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut xor = n as i32;
        for i in 0..n {
            xor ^= i as i32 ^ nums[i];
        }

        xor
    }
}

pub fn main() {
    let nums = vec![0, 1, 3, 4];
    println!("{}", Solution::missing_number_xor(nums));
}
