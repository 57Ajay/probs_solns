use std::{
    io::{Read, stdin},
    println, vec,
};

const MOD: u32 = 1_000_000_007;

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
        let c = v.parse::<usize>().unwrap();
        if c <= x {
            coins.push(c);
        }
    }
    if coins.is_empty() {
        println!("{}", 0);
        return;
    }

    let mut dp: Vec<u32> = vec![0; x + 1];
    dp[0] = 1;

    for &coin in &coins {
        for s in coin..=x {
            dp[s] += dp[s - coin];
            if dp[s] >= MOD {
                dp[s] -= MOD;
            }
        }
    }

    println!("{}", dp[x]);
}
