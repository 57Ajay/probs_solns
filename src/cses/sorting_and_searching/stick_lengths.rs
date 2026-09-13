use std::{io::Read, println};

pub fn main() {
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = match iter.next().and_then(|v| v.parse::<usize>().ok()) {
        Some(x) => x,
        None => return,
    };

    let mut p = Vec::with_capacity(n);

    while let Some(v) = iter.next() {
        p.push(v.parse::<usize>().unwrap());
    }

    p.sort_unstable();

    let midx = n / 2;
    let mv = p[midx];

    let mut cost = 0;

    for v in p {
        cost += mv.abs_diff(v);
    }

    println!("{}", cost);
}
