use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[usize; 26], usize> = HashMap::new();
        let mut groups: Vec<Vec<String>> = Vec::new();

        for s in strs {
            let mut count = [0usize; 26];

            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }

            let idx = *map.entry(count).or_insert_with(|| {
                groups.push(Vec::new());
                groups.len() - 1
            });

            groups[idx].push(s);
        }

        groups
    }
}

pub fn main() {
    let s = vec![
        "act".to_string(),
        "pots".to_string(),
        "tops".to_string(),
        "cat".to_string(),
        "stop".to_string(),
        "hat".to_string(),
    ];

    println!("{:?}", Solution::group_anagrams(s));
}
