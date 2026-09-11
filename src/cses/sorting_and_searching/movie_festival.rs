#![allow(unused)]
use std::{collections::BTreeMap, io::Read, println};

pub fn main() {
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = match iter.next().and_then(|v| v.parse::<usize>().ok()) {
        Some(n) => n,
        None => return,
    };

    let mut m = Vec::<(usize, usize)>::with_capacity(n);

    for _ in 0..n {
        let a = iter.next().unwrap().parse::<usize>().unwrap();
        let b = iter.next().unwrap().parse::<usize>().unwrap();
        m.push((a, b));
    }

    m.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut ce = 0;
    let mut max = 0;
    for (s, e) in m {
        if s >= ce {
            ce = e;
            max += 1;
        }
    }

    println!("{}", max);
}
