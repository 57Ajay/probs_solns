use std::{
    collections::{HashMap, hash_map},
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
    let mut current_unique_index = 1;
    let mut current_sub_array_count = 0;
    let mut ci = 1;

    while let Some(v) = iter.next() {
        let val = v.parse::<usize>().unwrap();

        match hmap.entry(val) {
            hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
            hash_map::Entry::Occupied(mut e) => {
                let v = e.get_mut();
                current_unique_index = 1 + *v;
                *v = ci;
            }
        }
        current_sub_array_count += ci - current_unique_index + 1;
        ci += 1;
    }

    println!("{current_sub_array_count}");
}
