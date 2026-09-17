use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufWriter, Read, Write, stdin, stdout},
    write,
};

pub fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let handle = stdout.lock();

    let mut writer = BufWriter::new(handle);
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let x = match iter.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => return,
    };

    let n = match iter.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut btset = BTreeSet::<usize>::new();
    let mut btmap = BTreeMap::<usize, usize>::new();

    btset.insert(0);
    btset.insert(x);
    btmap.insert(x, 1);

    for _ in 0..n {
        let val = iter.next().unwrap().parse::<usize>().unwrap();

        let start = *btset.range(..val).next_back().unwrap();
        let end = *btset.range(val..).next().unwrap();

        btset.insert(val);
        if let std::collections::btree_map::Entry::Occupied(mut e) = btmap.entry(end - start) {
            if *e.get() == 1 {
                e.remove();
            } else {
                *e.get_mut() -= 1;
            }
        }

        *btmap.entry(val - start).or_insert(0) += 1;
        *btmap.entry(end - val).or_insert(0) += 1;
        write!(writer, "{} ", btmap.keys().next_back().unwrap()).unwrap();
    }

    writer.flush().unwrap();
}
