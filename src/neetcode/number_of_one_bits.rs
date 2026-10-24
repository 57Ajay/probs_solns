pub struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut one_s = 0;

        while n > 0 {
            let m = n % 2;
            if m == 1 {
                one_s += 1;
            }
            n /= 2;
        }
        one_s
    }

    pub fn hamming_weight2(mut n: u32) -> i32 {
        let mut one_s = 0;

        while n > 0 {
            n &= n - 1;
            one_s += 1;
        }
        one_s
    }
}

pub fn main() {
    println!("{}", Solution::hamming_weight2(23));
}
