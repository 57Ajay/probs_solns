use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read the line");

    let mut current_max = 0;
    let mut current_char_max = 0;

    let input = input.as_bytes();

    // let mut char_under_inspection = input.chars().nth(0).or_else(|| Some('A')).unwrap();
    let mut char_under_inspection = input.get(0).unwrap();

    for i in 0..input.len() - 1 {
        let loop_char = input.get(i).unwrap();

        if loop_char == char_under_inspection {
            current_char_max += 1;
        } else {
            char_under_inspection = loop_char;
            current_char_max = 1;
        }

        if current_char_max > current_max {
            current_max = current_char_max;
        }
    }

    print!("{current_max}");
    // return current_max;
}
