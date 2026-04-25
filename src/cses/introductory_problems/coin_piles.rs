use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut test_array = String::new();
        io::stdin().read_line(&mut test_array).unwrap();
        let test_array = test_array
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        can_empty_piles(test_array[0], test_array[1]);
    }
}
fn can_empty_piles(a: usize, b: usize) {
    if (a + b) % 3 == 0 && 2 * a >= b && 2 * b >= a {
        println!("YES");
    } else {
        println!("NO");
    }
}

