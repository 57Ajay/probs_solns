use std::io;

const MOD: usize = 1_000_000_007;

#[allow(dead_code)]
pub fn main() {
    // very easy i guess using binary exponentiation
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    println!("{}", mod_pow(2, n, MOD));
}

fn mod_pow(mut base: usize, mut exp: usize, modu: usize) -> usize {
    let mut result = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modu;
        }
        base = (base * base) % modu;
        exp >>= 1;
    }

    result
}
