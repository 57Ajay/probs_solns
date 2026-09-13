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

    let mut x = Vec::with_capacity(n);
    while let Some(v) = iter.next() {
        let val = v.parse::<usize>().unwrap();
        x.push(val);
    }
    x.sort_unstable();

    let mut current_target: usize = 1;

    for &c in &x {
        if c > current_target {
            break;
        }
        current_target += c;
    }

    println!("{}", current_target);
}
