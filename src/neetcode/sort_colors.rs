pub struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let low = 0;
        let hi = nums.len() - 1;

        Solution::quick_sort(nums, low, hi);
    }
    fn partition(nums: &mut [i32], low: usize, hi: usize) -> usize {
        let pivot = nums[hi];
        let mut i = low;

        for j in low..hi {
            if nums[j] <= pivot {
                nums.swap(i, j);
                i += 1;
            }
        }

        nums.swap(i, hi);
        i
    }

    fn quick_sort(nums: &mut [i32], low: usize, hi: usize) {
        if low >= hi {
            return;
        }

        let p_idx = Self::partition(nums, low, hi);

        if p_idx > low {
            Self::quick_sort(nums, low, p_idx - 1);
        }

        if p_idx < hi {
            Self::quick_sort(nums, p_idx + 1, hi);
        }
    }
}

pub fn main() {
    let mut nums = vec![1, 0, 1, 2];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
