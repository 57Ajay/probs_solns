pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n = nums.len();
        let target = target as i64;

        let mut res = Vec::new();

        for a in 0..n {
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }

            for b in a + 1..n {
                if b > a + 1 && nums[b] == nums[b - 1] {
                    continue;
                }

                let need = target - nums[a] as i64 - nums[b] as i64;
                let (mut l, mut r) = (b + 1, n - 1);

                while l < r {
                    let sum = nums[l] as i64 + nums[r] as i64;
                    if sum == need {
                        res.push(vec![nums[a], nums[b], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    } else if sum < need {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }

        res
    }
}

pub fn main() {
    let nums = vec![3, 2, 3, -3, 1, 0];
    let target = 3;

    println!("{:?}", Solution::four_sum(nums, target));
}
