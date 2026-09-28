use std::{
    io::{Read, stdin},
    println, vec,
};

const MOD: u32 = 1000_000_007;

pub fn main() {
    let mut input = String::new();

    if stdin().lock().read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let size = n + 2;
    let mut grid = vec![vec![1u8; size]; size];

    let m = size - 1;
    for i in 0..size {
        grid[0][i] = 0;
        grid[i][0] = 0;
        grid[m][i] = 0;
        grid[i][m] = 0;
    }

    let mut i = 1;
    while let Some(row) = iter.next() {
        for ch in row.bytes() {
            let r = (i - 1) / n + 1;
            let c = (i - 1) % n + 1;

            if ch == b'*' {
                grid[r][c] = 0;
            }

            i += 1;
        }
    }

    let mut dp = vec![vec![0; size]; size];
    dp[1][1] = if grid[1][1] != 0 { 1 } else { 0 };
    dp[0][1] = 1;
    for r in 1..=n {
        for c in 1..=n {
            if grid[r][c] != 0 {
                dp[r][c] = (dp[r][c - 1] + dp[r - 1][c]) % MOD;
            }
        }
    }

    println!("{}", dp[n][n]);
}

#[allow(unused)]
fn dp_1d_sol() {
    let mut input = String::new();

    if stdin().lock().read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_ascii_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let size = n + 2;
    let mut grid = vec![vec![1u8; size]; size];

    let m = size - 1;
    for i in 0..size {
        grid[0][i] = 0;
        grid[i][0] = 0;
        grid[m][i] = 0;
        grid[i][m] = 0;
    }

    let mut i = 1;
    while let Some(row) = iter.next() {
        for ch in row.bytes() {
            let r = (i - 1) / n + 1;
            let c = (i - 1) % n + 1;

            if ch == b'*' {
                grid[r][c] = 0;
            }

            i += 1;
        }
    }

    let mut dp = vec![0; m];
    dp[1] = if grid[1][1] != 0 { 1 } else { 0 };

    for r in 1..=n {
        for c in 1..=n {
            if grid[r][c] != 0 {
                dp[c] = (dp[c - 1] + dp[c]) % MOD;
            } else {
                dp[c] = 0;
            }
        }
        println!("dp{r}: {:?}", dp);
    }

    println!("{}", dp[n]);
}
