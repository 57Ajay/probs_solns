use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let input = input
        .trim()
        .parse::<u32>()
        .expect("failed to parse into u32");

    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .expect("failed to read input");

    let mut numbers_vec: Vec<u32> = numbers
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    numbers_vec.sort();

    let mut _left_num = 0;

    let last_num = numbers_vec[numbers_vec.len() - 1];
    if input != last_num {
        print!("{input}");
    }

    for i in 1..=input - 1 {
        if i != *numbers_vec.get((i - 1) as usize).unwrap() {
            _left_num = i;
            print!("{_left_num}");
            break;
        }
    }
}
