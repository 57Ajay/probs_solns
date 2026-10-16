pub struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums;

        if !sorted_nums.is_empty() {
            let hi = sorted_nums.len() - 1;
            Self::quick_sort(&mut sorted_nums, 0, hi);
        }

        sorted_nums
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
    let nums = vec![10, 9, 1, 1, 1, 2, 3, 1];
    println!("{:?}", Solution::sort_array(nums));
}
