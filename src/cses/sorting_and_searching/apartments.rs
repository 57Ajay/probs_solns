use std::{io::Read, println};

pub fn main() {
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = match iter.next().and_then(|tok| tok.parse::<usize>().ok()) {
        Some(n) => n,
        None => return,
    };
    let m = match iter.next().and_then(|tok| tok.parse::<usize>().ok()) {
        Some(n) => n,
        None => return,
    };
    let k = match iter.next().and_then(|tok| tok.parse::<usize>().ok()) {
        Some(n) => n,
        None => return,
    };

    let mut daps = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(tok) = iter.next() {
            if let Ok(v) = tok.parse::<usize>() {
                daps.push(v);
            }
        } else {
            break;
        }
    }

    daps.sort_unstable();

    let mut aps = Vec::with_capacity(m);
    for _ in 0..m {
        if let Some(tok) = iter.next() {
            if let Ok(v) = tok.parse::<usize>() {
                aps.push(v);
            }
        } else {
            break;
        }
    }

    aps.sort_unstable();

    println!("{}", calculate(n, m, k, &daps, &aps));
}

pub fn calculate(n: usize, m: usize, k: usize, daps: &[usize], aps: &[usize]) -> usize {
    let mut total_possible = 0;
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        if daps[i] - k > aps[j] {
            j += 1;
        } else if aps[j] > daps[i] + k {
            i += 1;
        } else {
            if daps[i].abs_diff(aps[j]) <= k {
                i += 1;
                j += 1;
                total_possible += 1;
            }
        }
    }

    total_possible
}
