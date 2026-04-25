use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let input = input
        .trim()
        .parse::<usize>()
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

    assert_eq!(input, numbers_vec.len());

    let mut min_moves: u64 = 0;

    if numbers_vec.len() == 1 {
        print!("{}", 0);
        // return 0;
    } else {
        for i in 1..numbers_vec.len() {
            let x = numbers_vec[i];
            let y = numbers_vec[i - 1];
            let z = y - x;
            if x < y {
                min_moves += z as u64;
                numbers_vec[i] = y;
            }
        }

        print!("{min_moves}");

        // return min_moves;
    }

    // println!("{:?}", numbers_vec);
    // return 1;
}
