use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut n = input.trim().parse::<usize>().unwrap();

    let mut count = 0;
    while n > 0 {
        n /= 5;
        count += n;
    }

    println!("{}", count);
}
