pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let l = heights.len();
        let mut max: (usize, usize) = (0, l - 1);
        let mut max_area = 0;

        while max.0 < max.1 {
            let area = (max.1 - max.0) * (heights[max.0].min(heights[max.1])) as usize;
            if area > max_area {
                max_area = area;
            }
            if heights[max.0] > heights[max.1] {
                max.1 -= 1;
            } else {
                max.0 += 1;
            }
        }

        max_area as i32
    }
}

pub fn main() {
    let heights = vec![1, 7, 2, 5, 4, 7, 3, 6];
    println!("{}", Solution::max_area(heights));
}
