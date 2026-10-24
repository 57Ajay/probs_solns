pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut left_max = height[l];
        let mut right_max = height[r];
        let mut trapped_water = 0;

        while l < r {
            if left_max < right_max {
                l += 1;
                left_max = left_max.max(height[l]);
                trapped_water += left_max - height[l];
            } else {
                r -= 1;
                right_max = right_max.max(height[r]);
                trapped_water += right_max - height[r];
            }
        }

        trapped_water
    }
}

pub fn main() {
    let height = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1];

    println!("{:?}", Solution::trap(height));
}
