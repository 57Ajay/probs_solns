pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = s.len() - 1;
        for i in 0..(l + 1) / 2 {
            s.swap(i, l);
            l -= 1;
        }
    }
}

pub fn main() {
    let mut s = vec!['r', 'a', 'c', 'e', 'c'];
    Solution::reverse_string(&mut s);
    println!("{:?}", s);
}
