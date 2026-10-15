pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut idx = 0;
        for i in 0..len {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        for i in idx..len {
            nums[i] = val;
        }

        println!("nums: {:?}", nums);
        idx as i32
    }
}

pub fn main() {
    println!(
        "{:?}",
        Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2)
    );
}
