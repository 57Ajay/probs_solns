use std::io::{self, Write};

#[allow(dead_code)]
pub fn main() {
    let mut str_input = String::new();
    std::io::stdin()
        .read_line(&mut str_input)
        .expect("failed to read line");

    let mut input = str_input
        .trim()
        .parse::<u64>()
        .expect("not a valid input, only integer is supported");

    let mut output = String::with_capacity(4096);
    output.push_str(&input.to_string());

    loop {
        if input == 1 {
            break;
        }

        if input % 2 == 0 {
            input /= 2;
        } else {
            input = (input * 3) + 1;
        }

        output.push(' ');
        output.push_str(&input.to_string());
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    stdout.write_all(output.as_bytes()).unwrap();
}
