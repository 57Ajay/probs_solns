pub struct Solution;

impl Solution {
    pub fn reverse_bits(mut n: u32) -> u32 {
        let mut result = 0;
        for _ in 0..32 {
            let lowest_bit = n & 1;
            result = result << 1;
            result |= lowest_bit;
            n = n >> 1;
        }
        result
    }
}

pub fn main() {
    println!("{}", Solution::reverse_bits(21));
}
