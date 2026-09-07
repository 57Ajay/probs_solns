use std::{io::Read, println};

pub fn count_unique_keys<R: Read>(mut reader: R) -> usize {
    let mut input = String::new();
    if reader.read_to_string(&mut input).is_err() {
        return 0;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = match iter.next().and_then(|tok| tok.parse::<usize>().ok()) {
        Some(n) => n,
        None => return 0,
    };

    if n <= 1 {
        return n;
    }

    let mut vals: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(tok) = iter.next() {
            if let Ok(v) = tok.parse::<u32>() {
                vals.push(v);
            }
        } else {
            break;
        }
    }

    vals.sort_unstable();
    vals.dedup();
    vals.len()
}

pub fn main() {
    let stdin = std::io::stdin();
    let count = count_unique_keys(stdin.lock());
    println!("{count}");
}
