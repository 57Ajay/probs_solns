use std::{
    io::{Read, stdin},
    println, vec,
};

pub fn main() {
    let stdin = stdin();
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

    let mut x = vec![0; n + 1];
    let mut i = 0;
    while let Some(v) = iter.next() {
        let val = v.parse::<usize>().unwrap();
        x[val] = i;
        i += 1;
    }
    let mut rounds = 1;
    for i in 1..n {
        if x[i + 1] < x[i] {
            rounds += 1;
        }
    }
    println!("{rounds}");
}
