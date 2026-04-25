use std::io::{self, BufRead};

#[allow(dead_code)]
fn num_floating_window() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let sum = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    let mut count = 0;

    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    if numbers.len() < k {
        println!("max_sum: 0");
        println!("max_mult: 0");
        return;
    }

    let mut current_sum: u64 = numbers[..k].iter().sum();
    if current_sum == sum {
        count += 1;
    }
    let mut max_sum = current_sum;

    let mut zero_count = numbers[..k].iter().filter(|&&x| x == 0).count();
    let mut non_zero_product: u64 = numbers[..k]
        .iter()
        .filter(|&&x| x != 0)
        .fold(1u64, |acc, &x| acc.saturating_mul(x));

    let current_mult = if zero_count > 0 { 0 } else { non_zero_product };
    let mut max_mult = current_mult;

    for i in k..numbers.len() {
        current_sum = current_sum + numbers[i] - numbers[i - k];
        if current_sum == sum {
            count += 1;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }

        let outgoing = numbers[i - k];
        let incoming = numbers[i];

        if outgoing == 0 {
            zero_count -= 1;
        } else {
            non_zero_product /= outgoing;
        }

        if incoming == 0 {
            zero_count += 1;
        } else {
            non_zero_product = non_zero_product.saturating_mul(incoming);
        }

        let current_mult = if zero_count > 0 { 0 } else { non_zero_product };
        if current_mult > max_mult {
            max_mult = current_mult;
        }
    }

    println!("max_sum: {}", max_sum);
    println!("max_mult: {}", max_mult);
    println!("count: {}", count);
}

pub fn sliding_window() {
    num_floating_window()
}
