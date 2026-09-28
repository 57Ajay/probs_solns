#![allow(unused)]
use std::{
    io::{Read, stdin},
    println, vec,
};

pub fn main() {
    greedy_impl();
    // let mut input = String::new();
    //
    // let mut reader = stdin().lock();
    // if reader.read_to_string(&mut input).is_err() {
    //     return;
    // }
    //
    // let n = input.trim().parse::<usize>().unwrap();
    //
    // let mut dp = vec![1_000_000_000usize; n + 1];
    // dp[0] = 0;
    //
    // for i in 1..=n {
    //     let mut temp = i;
    //     while temp > 0 {
    //         let digit = temp % 10;
    //         if digit > 0 {
    //             dp[i] = dp[i].min(dp[i - digit] + 1);
    //         }
    //         temp /= 10;
    //     }
    // }
    //
    // println!("{}", dp[n]);
}

#[allow(unused)]
pub fn greedy_impl() {
    let mut input = String::new();
    if stdin().lock().read_to_string(&mut input).is_err() {
        return;
    }

    let mut n = input.trim().parse::<usize>().unwrap();
    let mut steps = 0;

    while n > 0 {
        let mut max_digit = 0;
        let mut temp = n;

        while temp > 0 {
            max_digit = max_digit.max(temp % 10);
            temp /= 10;
        }

        n -= max_digit;
        steps += 1;
    }

    println!("{steps}");
}
