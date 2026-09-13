use std::{io::Read, println};

pub fn main() {
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let _n = match iter.next().and_then(|v| v.parse::<usize>().ok()) {
        Some(x) => x,
        None => return,
    };

    let v1 = iter.next().unwrap().parse::<i64>().unwrap();
    let mut cm = v1;
    let mut gm = v1;

    while let Some(v) = iter.next() {
        let val = v.parse::<i64>().unwrap();
        cm = val.max(cm + val);
        gm = gm.max(cm);
    }

    println!("{}", gm);
}
