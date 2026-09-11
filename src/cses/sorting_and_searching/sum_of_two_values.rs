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

    let x = match iter.next().and_then(|v| v.parse::<usize>().ok()) {
        Some(y) => y,
        None => return,
    };

    let mut v = Vec::<(usize, usize)>::with_capacity(n);
    let mut idx = 1;
    while let Some(i) = iter.next() {
        v.push((i.parse().unwrap(), idx));
        idx += 1;
    }

    if v.len() == 1 {
        println!("IMPOSSIBLE");
        return;
    }

    v.sort_unstable();

    let mut l = 0;
    let mut r = v.len() - 1;

    while l < r {
        let sum = v[l].0 + v[r].0;
        if sum == x {
            println!("{} {}", v[l].1, v[r].1);
            return;
        } else if sum < x {
            l += 1;
        } else {
            r -= 1;
        }
    }

    println!("IMPOSSIBLE");
}
