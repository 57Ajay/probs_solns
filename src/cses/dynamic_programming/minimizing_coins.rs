use std::{
    io::{Read, stdin},
    println, vec,
};

pub fn main() {
    let mut input = String::new();
    if stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let x = iter.next().unwrap().parse::<usize>().unwrap();

    let mut coins = Vec::<usize>::with_capacity(n);

    while let Some(v) = iter.next() {
        coins.push(v.parse::<usize>().unwrap());
    }

    let mut s = vec![usize::MAX; x + 1];

    s[0] = 0;

    for i in 1..=x {
        for &coin in &coins {
            if coin <= i && s[i - coin] != usize::MAX {
                s[i] = s[i].min(s[i - coin] + 1);
            }
        }
    }

    if s[x] != usize::MAX {
        println!("{}", s[x]);
    } else {
        println!("{}", -1);
    }
}
