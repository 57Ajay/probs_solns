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
        .expect("failed to parse into usize");

    if input == 1 {
        println!("{}", 1);
    } else if input == 2 || input == 3 {
        println!("NO SOLUTION");
    } else {
        let mut i = input / 2 as usize;
        while i >= 1 {
            print!("{} {} ", i, i + (input / 2));
            i -= 1;
        }
        if input % 2 == 1 {
            print!("{input}\n");
        } else {
            print!("\n");
        }
    }
}
