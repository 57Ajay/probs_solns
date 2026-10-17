pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut str = String::new();

        for s in &strs {
            str.push_str(&s.len().to_string());
            str.push_str("#");
            str.push_str(s);
        }

        str
    }

    pub fn decode(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            let start = i;

            while bytes[i] != b'#' {
                i += 1;
            }

            let len: usize = std::str::from_utf8(&bytes[start..i])
                .unwrap()
                .parse()
                .unwrap();

            i += 1;

            let word = std::str::from_utf8(&bytes[i..i + len]).unwrap().to_string();

            result.push(word);

            i += len;
        }

        result
    }
}

pub fn main() {
    let strs = vec!["Hello".to_string(), "World".to_string()];
    let encoded = Solution::encode(strs);
    println!("{:?}", Solution::decode(encoded));
}
