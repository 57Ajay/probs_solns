use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    let mut w_buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    io::stdin().read_line(&mut w_buf).unwrap();

    let _n = buf.trim().parse::<usize>().unwrap();
    let arr = w_buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let total_sum: usize = arr.iter().sum();
    let target = total_sum / 2;

    let mut best = 0usize;
    let n = arr.len();

    for mask in 0..(1usize << n) {
        let mut s = 0usize;
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                s += arr[i];
            }
        }
        if s <= target && s > best {
            best = s;
        }
    }

    let other = total_sum - best;
    println!("{}", (other as i64 - best as i64).abs());
}

