pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let (mut i, mut j) = (a.len(), b.len());
        let mut carry = 0;
        let mut res = Vec::with_capacity(a.len().max(b.len()) + 1);

        while i > 0 || j > 0 || carry > 0 {
            let mut sum = carry;
            if i > 0 {
                i -= 1;
                sum += a[i] - b'0';
            }
            if j > 0 {
                j -= 1;
                sum += b[j] - b'0';
            }
            res.push(b'0' + sum % 2);
            carry = sum / 2;
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}

pub fn main() {
    let a = String::from("101");
    let b = String::from("10");
    println!("{}", Solution::add_binary(a, b))
}
