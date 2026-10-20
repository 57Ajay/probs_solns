pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        if b.is_empty() {
            return true;
        }
        let (mut i, mut j) = (0, b.len() - 1);

        while i < j {
            while i < j && !b[i].is_ascii_alphanumeric() {
                i += 1;
            }
            while i < j && !b[j].is_ascii_alphanumeric() {
                j -= 1;
            }
            if b[i].to_ascii_lowercase() != b[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j = j.saturating_sub(1);
        }
        true
    }
}

pub fn main() {
    println!(
        "{}",
        Solution::is_palindrome(String::from("No lemon, no melon"))
    );
}
