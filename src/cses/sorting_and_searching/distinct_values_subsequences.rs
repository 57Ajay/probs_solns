use std::{
    collections::HashMap,
    io::{Read, stdin},
    println,
};
const MOD: usize = 1_000_000_007;

pub fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = match iter.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut hmap = HashMap::<usize, usize>::with_capacity(n);

    while let Some(v) = iter.next() {
        let key = v.parse::<usize>().unwrap();
        hmap.entry(key).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut c = 1;
    for (_, v) in hmap.iter() {
        let choice = v + 1;
        c = (c * choice) % MOD;
    }
    c = (c + MOD - 1) % MOD;

    println!("{c}");
}
