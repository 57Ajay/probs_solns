pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0;

        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }

        (i + 1) as i32
    }
}

pub fn main() {
    let mut nums = vec![1, 1, 2, 3, 4];
    println!("{}", Solution::remove_duplicates(&mut nums));
    println!("nums: {:?}", nums);
}
