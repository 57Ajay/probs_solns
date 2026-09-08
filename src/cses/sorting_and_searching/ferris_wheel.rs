use std::{
    io::{Read, stdin},
    println,
};

pub fn main() {
    let stdin = stdin();
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

    let x = match iter.next().and_then(|tok| tok.parse::<usize>().ok()) {
        Some(n) => n,
        None => return,
    };

    let mut w = Vec::with_capacity(n);

    while let Some(weight) = iter.next() {
        w.push(weight.parse::<usize>().unwrap());
    }

    w.sort_unstable();

    println!("{}", get_min_num(x, &w));
}

pub fn get_min_num(x: usize, w: &[usize]) -> usize {
    if w.is_empty() {
        return 0;
    }

    let mut gondolas = 0;
    let mut i = 0;
    let mut j = w.len() - 1;

    while i <= j {
        if i < j && w[i] + w[j] <= x {
            i += 1;
        }
        gondolas += 1;
        if j == 0 {
            break;
        }
        j -= 1;
    }

    gondolas
}
