use std::{collections::HashMap, io};

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::with_capacity(8);
    io::stdin().read_line(&mut buf).unwrap();
    let buf = buf.trim().chars().collect::<Vec<char>>();

    let mut str_hmap = HashMap::<char, usize>::with_capacity(buf.len());

    for &c in &buf {
        *str_hmap.entry(c).or_insert(0) += 1;
    }

    let mut number_of_strings = factorial(buf.len());
    for (_, v) in str_hmap.iter() {
        number_of_strings /= factorial(*v);
    }
    println!("{}", number_of_strings);
    print_strings(buf);
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn print_strings(buf: Vec<char>) {
    let perms = permutations(buf);
    for i in perms {
        println!("{}", i);
    }
}

fn permutations(mut chars: Vec<char>) -> Vec<String> {
    chars.sort_unstable();
    let mut result = Vec::new();
    let mut used = vec![false; chars.len()];
    let mut current = Vec::with_capacity(chars.len());

    fn backtrack(
        chars: &[char],
        used: &mut [bool],
        current: &mut Vec<char>,
        result: &mut Vec<String>,
    ) {
        if current.len() == chars.len() {
            result.push(current.iter().collect());
            return;
        }

        for i in 0..chars.len() {
            if used[i] {
                continue;
            }
            if i > 0 && chars[i] == chars[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;
            current.push(chars[i]);
            backtrack(chars, used, current, result);
            current.pop();
            used[i] = false;
        }
    }

    backtrack(&chars, &mut used, &mut current, &mut result);
    result
}

// AI solution (not tested)
// fn next_permutation(a: &mut [u8]) -> bool {
//     if a.len() < 2 { return false; }
//
//     let mut i = a.len() - 2;
//
//     while a[i] >= a[i + 1] {
//         if i == 0 {
//             return false;
//         }
//         i -= 1;
//     }
//
//     let mut j = a.len() - 1;
//     while a[j] <= a[i] {
//         j -= 1;
//     }
//
//     a.swap(i, j);
//     a[i + 1..].reverse();
//     true
// }
//
//
// fn main() {
//     let mut s = b"ajay".to_vec();
//     s.sort();
//
//     loop {
//         println!("{}", String::from_utf8_lossy(&s));
//         if !next_permutation(&mut s) {
//             break;
//         }
//     }
// }
