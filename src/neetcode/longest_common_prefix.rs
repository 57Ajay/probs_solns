pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].as_bytes().to_vec();

        for s in strs.iter().skip(1) {
            let bytes = s.as_bytes();
            let common_len = prefix
                .iter()
                .zip(bytes.iter())
                .take_while(|(a, b)| a == b)
                .count();

            prefix.truncate(common_len);

            if prefix.is_empty() {
                break;
            }
        }

        String::from_utf8(prefix).unwrap()
    }
}
pub fn main() {
    let strs = vec![
        "dance".to_string(),
        "dag".to_string(),
        "danger".to_string(),
        "damage".to_string(),
    ];
    println!("{}", Solution::longest_common_prefix(strs))
}
