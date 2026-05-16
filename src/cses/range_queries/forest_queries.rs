use std::io::StdoutLock;
use std::io::{self, BufWriter, Read, Write};

#[allow(dead_code, unused)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let q = iter.next().unwrap().parse::<usize>().unwrap();
    let stride = n + 1;
    let mut s: Vec<i64> = vec![0i64; stride * stride];

    for i in 1..=n {
        let mut cs = iter.next().unwrap().chars();
        for j in 1..=n {
            let c = cs.next().unwrap();
            let cc = if c == '*' { 1 } else { 0 };
            let t = s[(i - 1) * stride + j];
            let l = s[i * stride + j - 1];
            let tl = s[(i - 1) * stride + (j - 1)];
            s[i * stride + j] = cc + t + l - tl;
        }
    }
    for _ in 0..q {
        let y1 = iter.next().unwrap().parse::<usize>().unwrap();
        let x1 = iter.next().unwrap().parse::<usize>().unwrap();
        let y2 = iter.next().unwrap().parse::<usize>().unwrap();
        let x2 = iter.next().unwrap().parse::<usize>().unwrap();
        get_no_of_trees(&(y1, x1, y2, x2), &s, stride, &mut out);
    }
}

fn get_no_of_trees(
    q: &(usize, usize, usize, usize),
    s: &[i64],
    stride: usize,
    stdout: &mut BufWriter<StdoutLock>,
) {
    let br = s[q.2 * stride + q.3];
    let tr = s[(q.0 - 1) * stride + q.3];
    let bl = s[q.2 * stride + q.1 - 1];
    let tl = s[(q.0 - 1) * stride + q.1 - 1];
    writeln!(stdout, "{}", br + tl - tr - bl).unwrap();
}

#[allow(dead_code, unused)]
pub fn hyper_optemized() {
    let mut input = Vec::with_capacity(1 << 20);
    io::stdin().read_to_end(&mut input).unwrap();
    let bytes: &[u8] = &input;
    let len = bytes.len();
    let mut pos = 0usize;

    macro_rules! read_int {
        () => {{
            while pos < len && bytes[pos] <= b' ' {
                pos += 1;
            }
            let mut n = 0u32;
            while pos < len && bytes[pos] > b' ' {
                n = n * 10 + (bytes[pos] - b'0') as u32;
                pos += 1;
            }
            n as usize
        }};
    }

    let n = read_int!();
    let q = read_int!();

    let stride = n + 1;
    let mut pref = vec![0u32; stride * stride];

    for i in 1..=n {
        while pos < len && bytes[pos] <= b' ' {
            pos += 1;
        }
        let row = unsafe { bytes.get_unchecked(pos..pos + n) };
        pos += n;

        let base = i * stride;
        let above = base - stride;

        unsafe {
            let p = pref.as_mut_ptr();
            for j in 1..=n {
                let cc = (*row.get_unchecked(j - 1) == b'*') as u32;
                let v = cc + *p.add(above + j) + *p.add(base + j - 1) - *p.add(above + j - 1);
                *p.add(base + j) = v;
            }
        }
    }

    let mut out: Vec<u8> = Vec::with_capacity(q * 8 + 16);

    for _ in 0..q {
        let y1 = read_int!();
        let x1 = read_int!();
        let y2 = read_int!();
        let x2 = read_int!();

        let ans: u32 = unsafe {
            let p = pref.as_ptr();
            (*p.add(y2 * stride + x2) + *p.add((y1 - 1) * stride + (x1 - 1)))
                - (*p.add((y1 - 1) * stride + x2) + *p.add(y2 * stride + (x1 - 1)))
        };

        if ans == 0 {
            out.push(b'0');
        } else {
            let mut buf = [0u8; 10];
            let mut idx = 10;
            let mut x = ans;
            while x > 0 {
                idx -= 1;
                buf[idx] = b'0' + (x % 10) as u8;
                x /= 10;
            }
            out.extend_from_slice(&buf[idx..]);
        }
        out.push(b'\n');
    }

    let stdout = io::stdout();
    let mut lock = stdout.lock();
    lock.write_all(&out).unwrap();
}
