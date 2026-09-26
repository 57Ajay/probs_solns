use std::{
    io::{Read, stdin},
    println, vec,
};

const MOD: usize = 1_000_000_007;

pub fn main() {
    let mut input = String::new();
    if stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let n: usize = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => return,
    };

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for j in 1..=6 {
            if i >= j {
                dp[i] = (dp[i] + dp[i - j]) % MOD;
            }
        }
    }

    println!("{}", dp[n]);
}
