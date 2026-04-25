use std::{collections::HashMap, io};

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().chars().collect::<Vec<char>>();
    print_palindrome(input);
}

fn print_palindrome(input: Vec<char>) {
    let mut input_map = HashMap::<char, usize>::with_capacity(input.len());

    for i in 0..input.len() {
        *input_map.entry(input[i]).or_insert(0) += 1;
    }

    let mut keeper = Keeper::new(input.len());
    let mut odds = Vec::<char>::with_capacity(2);

    for (k, v) in input_map.iter() {
        if v % 2 == 1 {
            odds.push(*k);
        }
        if odds.len() == 2 {
            break;
        }
    }

    if odds.len() > 1 {
        println!("NO SOLUTION");
        return;
    }

    if odds.len() == 1 {
        let mid = input.len() / 2;
        keeper.pelindrom_vec[mid] = odds[0];
    }

    // for (k, v) in input_map.iter() {
    //     println!("{} -> {}", *k, *v);
    // }

    input_map.iter().for_each(|(k, v)| {
        for _ in 0..*v / 2 {
            keeper.pelindrom_vec[keeper.left_idx] = *k;
            keeper.left_idx += 1;
            keeper.pelindrom_vec[keeper.right_idx] = *k;
            keeper.right_idx -= 1;
        }
    });

    keeper.print();
}

struct Keeper {
    pelindrom_vec: Vec<char>,
    left_idx: usize,
    right_idx: usize,
}

impl Keeper {
    fn new(capacity: usize) -> Self {
        Self {
            pelindrom_vec: vec!['0'; capacity],
            left_idx: 0,
            right_idx: capacity - 1,
        }
    }

    fn print(self) {
        for i in 0..self.pelindrom_vec.len() {
            print!("{}", self.pelindrom_vec[i]);
        }
        print!("\n");
    }
}

