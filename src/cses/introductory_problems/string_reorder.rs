use std::{collections::BTreeMap, io};

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let s = buf.trim();

    if let Some(result) = rearrange(s) {
        println!("{}", result);
    } else {
        println!("-1");
    }
}

fn rearrange(s: &str) -> Option<String> {
    let mut freq = BTreeMap::new();
    for b in s.bytes() {
        *freq.entry(b).or_insert(0) += 1;
    }

    let n = s.len();
    let mut out = Vec::with_capacity(n);
    let mut last_char: Option<u8> = None;

    for i in 0..n {
        let mut found = false;
        let remaining_len = (n - i) as i32;

        let keys: Vec<u8> = freq.keys().cloned().collect();
        for &ch in &keys {
            if Some(ch) == last_char {
                continue;
            }

            *freq.get_mut(&ch).unwrap() -= 1;

            if is_possible(&freq, remaining_len - 1) {
                out.push(ch);
                last_char = Some(ch);
                if *freq.get(&ch).unwrap() == 0 {
                    freq.remove(&ch);
                }
                found = true;
                break;
            } else {
                *freq.get_mut(&ch).unwrap() += 1;
            }
        }

        if !found {
            return None;
        }
    }

    Some(unsafe { String::from_utf8_unchecked(out) })
}

fn is_possible(freq: &BTreeMap<u8, usize>, remaining: i32) -> bool {
    for &count in freq.values() {
        if count > ((remaining + 1) / 2) as usize {
            return false;
        }
    }
    true
}

