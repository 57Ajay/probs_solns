use std::{
    collections::VecDeque,
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

    let n = input.trim().parse::<usize>().unwrap();
    let mut q = VecDeque::with_capacity(n);
    for i in 1..=n {
        q.push_back(i);
    }

    let mut skip = true;
    while let Some(v) = q.pop_front() {
        if skip {
            q.push_back(v);
            skip = false;
        } else {
            write!(writer, "{v} ").unwrap();
            skip = true;
        }
    }
    writer.flush().unwrap();
}
