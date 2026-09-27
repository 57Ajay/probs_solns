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
    coins.sort_unstable();

    let mut dp = vec![0u32; x + 1];
    dp[0] = 1;

    for i in 1..=x {
        let mut ways = 0;
        for &coin in &coins {
            if coin > i {
                break;
            }
            ways += dp[i - coin];
            if ways >= MOD {
                ways -= MOD;
            }
        }
        dp[i] = ways;
    }

    println!("{}", dp[x]);
}
