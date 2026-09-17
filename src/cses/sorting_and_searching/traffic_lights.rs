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

    let mut btset = BTreeSet::<(usize, usize)>::new();
    let mut btmap = BTreeMap::<usize, usize>::new();
    let v1 = iter.next().unwrap().parse::<usize>().unwrap();
    btset.insert((0, v1));
    btset.insert((v1, x));

    update_count(&mut btmap, v1, UCOUNT::INC);
    update_count(&mut btmap, x - v1, UCOUNT::INC);

    write!(writer, "{} ", btmap.keys().next_back().unwrap()).unwrap();

    for _ in 1..n {
        let val = iter.next().unwrap().parse::<usize>().unwrap();

        if let Some(&x) = btset.range(..(val, 0)).next_back() {
            btset.remove(&x);
            btset.insert((x.0, val));
            btset.insert((val, x.1));
            let b = x.1 - x.0;
            update_count(&mut btmap, b, UCOUNT::DEC);

            let d1 = val - x.0;
            let d2 = x.1 - val;
            update_count(&mut btmap, d1, UCOUNT::INC);
            update_count(&mut btmap, d2, UCOUNT::INC);

            write!(writer, "{} ", btmap.keys().next_back().unwrap()).unwrap();
        }
    }

    writer.flush().unwrap();
}

enum UCOUNT {
    DEC,
    INC,
}

fn update_count(map: &mut BTreeMap<usize, usize>, val: usize, op: UCOUNT) {
    match map.get_mut(&val) {
        Some(count) => match op {
            UCOUNT::DEC => {
                if *count <= 1 {
                    map.remove(&val);
                } else {
                    *count -= 1;
                }
            }
            UCOUNT::INC => {
                *count += 1;
            }
        },
        None => {
            map.insert(val, 1);
        }
    }
}
