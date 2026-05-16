use std::io::{self, BufWriter, Read, Write};

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_ascii_whitespace();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let q = iter.next().unwrap().parse::<usize>().unwrap();

    let mut prefix = vec![0i64; n + 1];

    for i in 0..n {
        prefix[i + 1] = prefix[i] ^ iter.next().unwrap().parse::<i64>().unwrap();
    }

    for _ in 0..q {
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let ans = prefix[r] ^ prefix[l - 1];
        writeln!(out, "{}", ans).unwrap();
    }
}
