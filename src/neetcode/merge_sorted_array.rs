pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j) = (m as usize, n as usize);
        let mut k = (m + n) as usize;

        while j > 0 {
            k -= 1;
            if i > 0 && nums1[i - 1] > nums2[j - 1] {
                nums1[k] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[k] = nums2[j - 1];
                j -= 1;
            }
        }
    }
}

pub fn main() {
    let mut nums1 = vec![0, 0];
    let m = 0;
    let mut nums2 = vec![1, 2];
    let n = 2;
    Solution::merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1);
}
