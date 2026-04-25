use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let qn = n.trim().parse::<usize>().unwrap();

    let mut qs_vec = Vec::<usize>::with_capacity(qn);

    for _ in 0..qn {
        n.clear();
        io::stdin().read_line(&mut n).unwrap();
        let q = n.trim().parse::<usize>().unwrap();
        qs_vec.push(q);
    }

    for q in &mut qs_vec {
        corr_digt(q);
    }
}

fn corr_digt(q: &mut usize) {
    if *q < 10 {
        println!("{}", q);
        return;
    }

    let mut d = 1;

    for i in 1..=digits(*q as u64) as usize {
        if *q < totl_dig(i) - totl_dig(i - 1) {
            break;
        }
        d += 1;
        *q -= totl_dig(i) - totl_dig(i - 1);
        // println!("q_now: {}", *q);
    }
    // println!("q: {}", *q);

    let mut pm = max_n_digit(d - 1);

    pm += *q / d as usize;
    if *q % d as usize != 0 {
        pm += 1;
    }

    println!("{}", special_digit(pm, *q % d as usize));
    return;
}
fn digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn totl_dig(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut d: usize = 0;
    for i in 1..=n {
        d += (9 * i * 10usize.pow(i as u32 - 1)) as usize;
    }
    d
}

fn max_n_digit(n: u32) -> usize {
    10usize.pow(n) - 1
}
//
// fn ith_digit(num: usize, n: u32, i: u32) -> usize {
//     (num / 10usize.pow(n - i)) % 10
// }

fn special_digit(num: usize, i: usize) -> usize {
    if i == 0 {
        return num % 10;
    }

    let mut temp = num;
    let mut digits = 0;

    while temp > 0 {
        digits += 1;
        temp /= 10;
    }

    if i > digits {
        panic!("index out of bounds");
    }

    (num / 10usize.pow(digits as u32 - i as u32)) % 10
}

// fn min_n_digit(n: u32) -> u32 {
//     (10u32.pow(n) + 1) % 10
// }
