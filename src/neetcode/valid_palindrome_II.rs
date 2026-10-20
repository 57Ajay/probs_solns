pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        let (mut i, mut j) = (0, b.len().saturating_sub(1));

        while i < j {
            if b[i] != b[j] {
                println!("{:?}", Solution::is_pal(&b[i..j]));
                println!("{:?}", &b[i + 1..=j]);
                return Solution::is_pal(&b[i + 1..=j]) || Solution::is_pal(&b[i..j]);
            }
            i += 1;
            j -= 1;
        }
        true
    }

    fn is_pal(b: &[u8]) -> bool {
        let n = b.len();
        (0..n / 2).all(|k| b[k] == b[n - 1 - k])
    }
}

pub fn main() {
    println!("{}", Solution::valid_palindrome(String::from("adadaea")));
}
