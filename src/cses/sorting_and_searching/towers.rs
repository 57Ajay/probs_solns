use std::{
    collections::BTreeSet,
    io::{Read, stdin},
    ops::Bound::Excluded,
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

    let n = match iter.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut btset = BTreeSet::<(usize, usize)>::new();
    btset.insert((iter.next().unwrap().parse().unwrap(), 0));

    let mut towers = 1;
    for i in 1..n {
        let v = iter.next().unwrap().parse::<usize>().unwrap();
        let next_greater = btset
            .range((Excluded((v, usize::MAX)), std::ops::Bound::Unbounded))
            .next()
            .cloned();

        if let Some(val) = next_greater {
            btset.remove(&val);
        } else {
            towers += 1;
        }
        btset.insert((v, i));
    }

    println!("{towers}");
}
