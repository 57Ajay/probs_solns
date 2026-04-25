use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let buf = buf.trim().parse::<usize>().unwrap();
    print_steps(buf);
}

fn print_steps(disks: usize) {
    // two simple rules ->
    // 1. if an odd number of disks:
    // disk 1(smallest disk) moves in this direction: A → C → B → A → C → B → ...
    // 2. if have an even number of disks:
    // disk 1 moves like this: A → B → C → A → B → C → ...

    let mut a = Vec::<usize>::with_capacity(disks);
    let mut b = Vec::<usize>::with_capacity(disks);
    let mut c = Vec::<usize>::with_capacity(disks);

    for i in (1..=disks).rev() {
        a.push(i);
    }

    let total_moves = mod_pow(2, disks) - 1;
    println!("{}", total_moves);

    let (x, mut y, mut z) = ("1", "2", "3");

    if disks % 2 == 0 {
        std::mem::swap(&mut y, &mut z);
    }

    for i in 1..=total_moves {
        if i % 3 == 1 {
            make_legal_move(&mut a, &mut c, x, z);
        } else if i % 3 == 2 {
            make_legal_move(&mut a, &mut b, x, y);
        } else {
            make_legal_move(&mut b, &mut c, y, z);
        }
    }
}

fn make_legal_move(from: &mut Vec<usize>, to: &mut Vec<usize>, from_label: &str, to_label: &str) {
    let f_top = from.last().copied();
    let t_top = to.last().copied();

    match (f_top, t_top) {
        (None, None) => unreachable!(),

        (None, Some(_)) => {
            let d = to.pop().unwrap();
            from.push(d);
            println!("{} {}", to_label, from_label);
        }

        (Some(_), None) => {
            let d = from.pop().unwrap();
            to.push(d);
            println!("{} {}", from_label, to_label);
        }

        (Some(f), Some(t)) => {
            if f < t {
                let d = from.pop().unwrap();
                to.push(d);
                println!("{} {}", from_label, to_label);
            } else {
                let d = to.pop().unwrap();
                from.push(d);
                println!("{} {}", to_label, from_label);
            }
        }
    }
}

fn mod_pow(mut base: usize, mut exp: usize) -> usize {
    let mut result = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result *= base;
        }
        base *= base;
        exp >>= 1;
    }

    result
}
