use std::{
    io::{BufWriter, Read, Write, stdin, stdout},
    vec, writeln,
};

pub fn main() {
    let stdin = stdin();
    let mut reader = stdin.lock();
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);

    let mut input = String::new();

    if reader.read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = match iter.next().and_then(|v| v.parse::<usize>().ok()) {
        Some(x) => x,
        None => return,
    };
    let m = match iter.next().and_then(|v| v.parse::<usize>().ok()) {
        Some(x) => x,
        None => return,
    };

    let mut pos = vec![0; n + 1];
    let mut x = vec![0; n];

    for i in 0..n {
        let val = iter.next().unwrap().parse::<usize>().unwrap();
        pos[val] = i;
        x[i] = val;
    }

    let mut q = Vec::<(usize, usize)>::with_capacity(m);
    for _ in 0..m {
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        q.push((a, b));
    }

    let mut rounds = 1;
    for i in 1..n {
        if pos[i + 1] < pos[i] {
            rounds += 1;
        }
    }

    let mut cp = Vec::with_capacity(4);

    for (a, b) in q {
        cp.clear();
        if a == b {
            writeln!(writer, "{rounds}").unwrap();
            continue;
        }

        let v1 = x[a - 1];
        let v2 = x[b - 1];

        if v1 > 1 {
            cp.push((v1 - 1, v1));
        }
        if v1 < n {
            cp.push((v1, v1 + 1));
        }
        if v2 > 1 {
            cp.push((v2 - 1, v2));
        }
        if v2 < n {
            cp.push((v2, v2 + 1));
        }

        cp.sort_unstable();
        cp.dedup();

        for p in &cp {
            if pos[p.1] < pos[p.0] {
                rounds -= 1;
            }
        }
        pos.swap(v1, v2);
        x.swap(a - 1, b - 1);

        for p in &cp {
            if pos[p.1] < pos[p.0] {
                rounds += 1;
            }
        }

        writeln!(writer, "{rounds}").unwrap();
    }

    writer.flush().unwrap();
}
