use std::{io, ops::BitXor};

#[allow(dead_code)]
pub fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().parse::<usize>().unwrap();
    let n = mod_pow(2, buffer);
    for i in 0..n {
        print_grey_code(i, buffer);
    }
}

fn mod_pow(mut base: usize, mut exp: usize) -> usize {
    let mut result = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base;
        }
        base = base * base;
        exp >>= 1;
    }

    result
}
// 0 -> 00 -> 00
// 1 -> 01 -> 01
// 2 -> 10 -> 11
// 3 -> 11 -> 10

fn print_grey_code(n: usize, max: usize) {
    let x = format!("{:b}", n.bitxor(n >> 1));
    if x.len() < max {
        let mut s = String::new();
        for _ in 0..max - x.len() {
            s.push('0');
        }
        s.push_str(x.as_str());
        println!("{}", s);
        return;
    }
    println!("{}", x);
}

// AI solution (not tested)
// use std::io::{self, Write};
//
// pub fn main() {
//     // Read n
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).unwrap();
//     let n: usize = buffer.trim().parse().unwrap();
//
//     // Pre-allocate stdout buffer
//     let mut out = String::with_capacity((1 << n) * (n + 1));
//
//     let limit = 1usize << n;
//     for i in 0..limit {
//         let g = i ^ (i >> 1);
//         for bit in (0..n).rev() {
//             out.push(if (g >> bit) & 1 == 1 { '1' } else { '0' });
//         }
//         out.push('\n');
//     }
//
//     // flush once
//     print!("{out}");
// }
