pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max = (heights[0], 0);
        let mut max_area = 0;

        for (i, &h) in heights.iter().enumerate().skip(1) {
            let c_area = h.min(max.0) * (i.abs_diff(max.1) as i32);

            let h_diff = h.abs_diff(max.0);
            let w_diff = i - max.1;
            let ratio = h_diff as f32 / w_diff as f32;

            if ratio > 1.0 {
                if h > max.0 {
                    max = (h, i);
                }
            }

            if c_area > max_area {
                max_area = c_area;
            }
        }

        max_area
    }
}

pub fn main() {
    let heights = vec![1, 7, 2, 5, 4, 7, 3, 6];
    println!("{}", Solution::max_area(heights));
}
