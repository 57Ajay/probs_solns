pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev = 0i32;
        while x != 0 {
            let digit = x % 10;
            x /= 10;
            rev = match rev.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                Some(r) => r,
                None => return 0,
            };
        }
        rev
    }

    pub fn reverse_bitwise(x: i32) -> i32 {
        let sign = x >> 31;
        let mut n = ((x ^ sign).wrapping_sub(sign) as u32) as u64;
        let mut rev = 0u64;

        while n != 0 {
            rev = rev * 10 + n % 10;
            n /= 10;
        }

        let limit = i32::MAX as u64 + (sign & 1) as u64;
        if rev > limit {
            return 0;
        }
        (rev as i32 ^ sign).wrapping_sub(sign)
    }
}

pub fn main() {
    println!("{}", Solution::reverse(1234236467));
    println!("{}", Solution::reverse_bitwise(-123));
}
