use std::io;

use std::fs;

#[allow(dead_code)]
fn check_diff() {
    let my = fs::read_to_string("./user_output.txt").unwrap();
    let correct = fs::read_to_string("./test_output.txt").unwrap();

    for (i, (a, b)) in my.lines().zip(correct.lines()).enumerate() {
        if a != b {
            println!("Mismatch at line {}:", i + 1);
            println!("mine    : {}", a);
            println!("correct : {}", b);
            break;
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let v: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        judge(v);
    }
}

#[allow(dead_code)]
fn judge(v: Vec<usize>) {
    let n = v[0];
    let s1 = v[1];
    let s2 = v[2];

    if s1 + s2 > n {
        println!("NO");
        return;
    }

    if s1 + s2 == 1 {
        println!("NO");
        return;
    }

    if s1 + s2 > 0 && (s1 == 0 || s2 == 0) {
        println!("NO");
        return;
    }

    println!("YES");

    let mut a: Vec<usize> = (1..=n).collect();
    let mut b = a.clone();

    let min = s1.min(s2);
    for i in 0..min * 2 {
        if i % 2 == 0 {
            b.swap(i, i + 1);
        }
    }
    // println!("a: {:?}", a);
    // println!("b: {:?}", b);
    if s1.abs_diff(s2) > 0 {
        let base = min * 2;

        if s1 > s2 {
            for i in 0..s1.abs_diff(s2) {
                b.swap(base + i, base - 2);
            }
        } else {
            for i in 0..s1.abs_diff(s2) {
                a.swap(base + i, base - 1);
            }
        }
    }

    for i in a {
        print!("{} ", i);
    }
    println!();
    for i in b {
        print!("{} ", i);
    }
    println!();
    return;
}

