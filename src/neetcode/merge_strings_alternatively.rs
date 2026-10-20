pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::with_capacity(word1.len() + word2.len());
        let (mut w1, mut w2) = (word1.chars(), word2.chars());

        loop {
            match (w1.next(), w2.next()) {
                (Some(a), Some(b)) => {
                    res.push(a);
                    res.push(b);
                }
                (Some(a), None) => {
                    res.push(a);
                    res.push_str(w1.as_str());
                    break;
                }
                (None, Some(b)) => {
                    res.push(b);
                    res.push_str(w2.as_str());
                    break;
                }
                (None, None) => break,
            }
        }
        res
    }
}
pub fn main() {
    let word1 = String::from("ab");
    let word2 = String::from("abbxxc");

    println!("{}", Solution::merge_alternately(word1, word2));
}
