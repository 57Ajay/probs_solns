use std::{
    collections::{HashMap, hash_map::Entry},
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

    let n = match iter.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => return,
    };
    let mut hmap = HashMap::<usize, usize>::with_capacity(n);

    let mut cm = 0;
    let mut gm = 0;

    let mut i = 1usize;
    let mut s = 1usize;

    while let Some(v) = iter.next() {
        let val = v.parse::<usize>().unwrap();
        match hmap.entry(val) {
            Entry::Vacant(e) => {
                e.insert(i);
                cm += 1;
            }
            Entry::Occupied(mut e) => {
                let v = e.get_mut();
                let d = i - *v;
                if s <= *v {
                    s = *v + 1;
                    cm = d;
                } else {
                    cm += 1;
                }
                *v = i;
            }
        }
        i += 1;
        if gm < cm {
            gm = cm;
        }
    }

    println!("{gm}");
}
