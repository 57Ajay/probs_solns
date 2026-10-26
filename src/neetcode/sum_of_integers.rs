pub struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = (a & b) << 1;
            a ^= b;
            b = carry;
        }
        a
    }
}
pub fn main() {
    println!("{}", Solution::get_sum(2, 3));
}
