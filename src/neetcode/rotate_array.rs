pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        Solution::rotate_right(nums, k as usize);
    }

    fn rotate_right(a: &mut [i32], k: usize) {
        let n = a.len();

        if n == 0 {
            return;
        }

        let k = k % n;

        Solution::reverse(a, 0, n);
        Solution::reverse(a, 0, k);
        Solution::reverse(a, k, n);
    }

    fn reverse(a: &mut [i32], mut l: usize, mut r: usize) {
        while l + 1 < r {
            r -= 1;
            let tmp = a[l];
            a[l] = a[r];
            a[r] = tmp;
            l += 1;
        }
    }
}

pub fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8]; // [6, 7, 8, 1, 2, 3, 4, 5]
    let k = 4;
    Solution::rotate(&mut nums, k);
    println!("{:?}", nums);
}
