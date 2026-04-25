use std::io;

fn print_knights_do_not_attack(board_side: usize) -> usize {
    if board_side == 1 {
        return 0;
    }
    let total_squares = board_side * board_side;
    let total_placement = ((total_squares) * (total_squares - 1)) / 2;

    let total_ways_to_attack = 4 * (board_side - 1) * (board_side - 2);

    return total_placement - total_ways_to_attack;
}

#[allow(dead_code)]
pub fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().parse::<usize>().unwrap();

    for i in 1..=buffer {
        let res = print_knights_do_not_attack(i);
        println!("{}", res);
    }
}
